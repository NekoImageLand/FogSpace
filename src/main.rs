mod context;
mod errors;
mod ext;
mod handlers;
mod models;
mod routers;

use crate::routers::prelude::*;
use context::FogSpaceCtx;
use czkawka_core::common::set_config_cache_path;
use salvo::prelude::*;
use std::sync::Arc;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    set_config_cache_path("Czkawka", "Czkawka");
    let fog_ctx = Arc::new(FogSpaceCtx::default());
    let router = Router::with_path("tasks")
        .hoop(affix_state::inject(fog_ctx.clone()))
        .push(
            Router::with_path("create")
                .push(Router::with_path("similar_image").post(similar_image_task)),
        )
        .push(Router::with_path("cancel").post(cancel_task));
    // TODO: cancel route
    tracing::debug!("Router: \n {:?}", router);
    // TODO: dynamic address
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
