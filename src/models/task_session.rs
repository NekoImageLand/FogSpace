use crate::models::process_ref::ProgressDataRef;
use crate::models::task_result::TaskResultEnum;
use czkawka_core::progress_data::ProgressData;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
#[serde(tag = "status", content = "data")]
pub enum TaskStatus<'a, RT>
where
    RT: Serialize,
{
    Processing {
        task_id: Uuid,
        #[serde(with = "ProgressDataRef")]
        progress_data: ProgressData,
    },
    Processed {
        task_id: Uuid,
        result: TaskResultEnum<'a, RT>,
    },
    Error {
        task_id: Uuid,
        msg: String,
    },
}
