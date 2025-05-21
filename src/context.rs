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
    pub fn get(depot: &mut Depot) -> Self {
        match depot.obtain::<FogSpaceCtx>() {
            Ok(ctx_ref) => ctx_ref.clone(),
            Err(_) => {
                let ctx = FogSpaceCtx::default();
                depot.inject(ctx.clone());
                ctx
            }
        }
    }
}
