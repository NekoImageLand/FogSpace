use crate::errors::FogSpaceError;
use crate::ext::ToPathBufs;
use crate::handlers::HandlerCtx;
use crate::models::task_result::{HasRefResult, TaskInnerResult};
use crate::routers::similar_image::SearchParams;
use czkawka_core::common::set_number_of_threads;
use czkawka_core::common_tool::CommonData;
use czkawka_core::tools::similar_images::{
    ImagesEntry, SimilarImages, SimilarImagesParameters, return_similarity_from_similarity_preset,
};
use salvo::http::StatusCode;
use std::sync::Once;
use std::thread;

static RAYON_INIT: Once = Once::new();

impl<'a> HasRefResult<'a, Vec<Vec<ImagesEntry>>> for SimilarImages {
    fn result_ref(&'a self) -> &'a Vec<Vec<ImagesEntry>> {
        self.get_similar_images()
    }
}

impl HandlerCtx {
    pub fn calculate_similar_image(
        self,
        req: SearchParams,
    ) -> Result<TaskInnerResult<SimilarImages>, FogSpaceError> {
        let similarity =
            return_similarity_from_similarity_preset(&req.similarity_preset, req.hash_size as u8);
        let params = SimilarImagesParameters::new(
            similarity,
            req.hash_size as u8,
            req.hash_algorithm,
            req.image_filter,
            req.ignore_same_size,
            !req.allow_hard_links,
        );
        let calculate_thread =
            thread::Builder::new()
                .stack_size(8 * 1024 * 1024)
                .spawn(move || {
                    self.task(|stop_signal, process_sender| {
                        let mut op = SimilarImages::new(params);
                        RAYON_INIT.call_once(|| {
                            set_number_of_threads(req.common_items.thread_number as usize); // TODO: only once
                        });
                        // #region set_common_settings
                        let included_directories = req.common_items.directories.to_pathbufs();
                        op.set_included_directory(included_directories);
                        if let Some(ref ed) = req.common_items.excluded_directories {
                            op.set_excluded_directory(ed.to_pathbufs());
                        }
                        if let Some(ei) = req.common_items.excluded_items {
                            op.set_excluded_items(ei);
                        }
                        op.set_recursive_search(req.common_items.is_recursive);
                        if let Some(ae) = req.common_items.allowed_extensions {
                            op.set_allowed_extensions(ae.join(","));
                        }
                        op.set_use_cache(req.common_items.use_cache);
                        // #endregion set_common_settings
                        op.set_minimal_file_size(req.minimal_file_size);
                        op.set_maximal_file_size(req.maximal_file_size);
                        op.set_delete_method(req.delete_method);
                        op.set_dry_run(req.dry_run);

                        op.find_similar_images(Some(&stop_signal), Some(&process_sender));
                        let text_message = op.get_text_messages().create_messages_text();
                        TaskInnerResult::<SimilarImages> {
                            msg: Some(text_message),
                            op,
                        }
                    })
                });
        calculate_thread
            .map_err(|_| FogSpaceError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "calculate_thread Join Error!".to_string(),
            })?
            .join()
            .map_err(|_| FogSpaceError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "calculate_thread Join Error!".to_string(),
            })
    }
}
