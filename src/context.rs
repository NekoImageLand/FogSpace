use crate::errors::FogSpaceError;
use dashmap::DashMap;
use salvo::Depot;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct FogSpaceCtx {
    pub tasks: Arc<DashMap<Uuid, Arc<AtomicBool>>>,
}

impl FogSpaceCtx {
    pub fn get(depot: &mut Depot) -> Result<Arc<Self>, FogSpaceError> {
        depot
            .obtain::<Arc<FogSpaceCtx>>()
            .map_err(|_| FogSpaceError {
                code: salvo::http::StatusCode::INTERNAL_SERVER_ERROR,
                message: "failed to obtain FogSpaceCtx".to_string(),
            })
            .cloned()
    }
}
