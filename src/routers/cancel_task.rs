use std::sync::atomic::Ordering;

use crate::{context::FogSpaceCtx, errors::FogSpaceError};
use salvo::prelude::*;
use uuid::Uuid;

#[handler]
pub async fn cancel_task(
    req: &mut Request,
    _: &mut Response,
    depot: &mut Depot,
) -> Result<(), FogSpaceError> {
    let task_id: Uuid = req
        .query::<&str>("task_id")
        .ok_or(FogSpaceError {
            code: StatusCode::BAD_REQUEST,
            message: "Missing task_id".to_string(),
        })?
        .parse()
        .map_err(|_| FogSpaceError {
            code: StatusCode::BAD_REQUEST,
            message: "Invalid task_id".to_string(),
        })?;
    let fog_ctx = FogSpaceCtx::get(depot)?;
    if let Some(task) = fog_ctx.tasks.get(&task_id) {
        task.store(true, Ordering::Relaxed);
    }
    Ok(())
}
