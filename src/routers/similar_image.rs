use crate::context::FogSpaceCtx;
use crate::errors::FogSpaceError;
use crate::handlers::HandlerCtx;
use crate::models::common_params::{CommonParams, DeleteMethodRef, SimilarityPresetDef};
use crate::models::task_session::TaskStatus;
use crate::routers::bridge_crossbeam_to_tokio;
use async_stream::stream;
use czkawka_core::common_tool::DeleteMethod;
use czkawka_core::progress_data::ProgressData;
use czkawka_core::tools::similar_images::{ImagesEntry, SimilarityPreset};
use image::imageops::FilterType;
use image_hasher::HashAlg;
use salvo::http::StatusCode;
use salvo::macros::Extractible;
use salvo::prelude::*;
use salvo::{Depot, Request, Response, handler};
use serde::Deserialize;
use serde_aux::field_attributes::{bool_true, default_u32, default_u64};
use std::convert::Infallible;
use tokio::sync::oneshot;
use uuid::Uuid;

#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct SearchParams {
    #[salvo(extract(flatten))]
    pub common_items: CommonParams,
    // pub reference_directories: Option<Vec<String>>,  // TODO:
    #[serde(default = "default_u64::<16384>")]
    pub minimal_file_size: u64,
    #[serde(default = "default_u64::<18446744073709551615>")]
    pub maximal_file_size: u64,
    #[serde(with = "SimilarityPresetDef")]
    pub similarity_preset: SimilarityPreset,
    #[serde(with = "DeleteMethodRef")]
    pub delete_method: DeleteMethod,
    pub allow_hard_links: bool,
    #[serde(default = "bool_true")]
    pub dry_run: bool,
    pub ignore_same_size: bool,
    pub hash_algorithm: HashAlg,
    pub image_filter: FilterType,
    #[serde(default = "default_u32::<16>")]
    pub hash_size: u32,
}

#[handler]
pub async fn similar_image_task(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> Result<(), FogSpaceError> {
    let params: SearchParams = req.extract().await.map_err(|e| FogSpaceError {
        code: StatusCode::BAD_REQUEST,
        message: format!("invalid body: {e}"),
    })?;
    tracing::debug!("search handle! params: {:?}", params);
    let (progress_tx, progress_rx) = crossbeam_channel::unbounded::<ProgressData>();
    let req_uuid = Uuid::new_v4();
    let fog_ctx = FogSpaceCtx::get(depot)?;
    let handler_ctx = HandlerCtx::new(fog_ctx, progress_tx, req_uuid);
    let (done_tx, done_rx) = oneshot::channel::<Result<_, FogSpaceError>>();
    tokio::task::spawn_blocking(move || {
        let r = handler_ctx.calculate_similar_image(params);
        let _ = done_tx.send(r);
    });
    let mut tokio_progress_rx = bridge_crossbeam_to_tokio(progress_rx);
    let event_stream = stream! {
        while let Some(progress) = tokio_progress_rx.recv().await {
            let status: TaskStatus<'_, Vec<Vec<ImagesEntry>>>  = TaskStatus::Processing {
                task_id: req_uuid,
                progress_data: progress,
            };
            yield Ok::<_, Infallible>(SseEvent::default().json(&status).unwrap());
        }
        match done_rx.await {
            Ok(Ok(inner_res_enum)) => {
                let status = TaskStatus::Processed {
                    task_id: req_uuid,
                    result: inner_res_enum.to_ref_result(),
                };
                let evt = SseEvent::default().json(&status).unwrap();
                yield Ok(evt);
            }
            Ok(Err(e)) => {
                let status: TaskStatus<'_, Vec<Vec<ImagesEntry>>> = TaskStatus::Error {
                    task_id: req_uuid,
                    msg: e.to_string(),
                };
                let evt = SseEvent::default().json(&status).unwrap();
                yield Ok(evt);
            }
            Err(join_err) => {
                let status: TaskStatus<'_, Vec<Vec<ImagesEntry>>> = TaskStatus::Error {
                    task_id: req_uuid,
                    msg: format!("failed to receive result: {join_err}"),
                };
                let evt = SseEvent::default().json(&status).unwrap();
                yield Ok(evt);
            }
        }
    };
    SseKeepAlive::new(event_stream).stream(res);
    Ok(())
}
