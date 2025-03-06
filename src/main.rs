use axum::{middleware, routing::get, Router};

use blog_api::auth::*;
use blog_api::*;
use std::net::SocketAddr;

const VERSION_ONE: &'static str = "/api/v1/";

#[tokio::main]
async fn main() {
    // build our application with a single route
    let state = AppState::builder().pool(get_connection_pool()).build();

    let app = Router::new()
        .route(
            &(VERSION_ONE.to_owned() + "blogs/{payload}"),
            get(get_post).patch(update_post).delete(delete_post),
        )
        .route(
            &(VERSION_ONE.to_owned() + "blogs"),
            get(get_posts).post(add_post),
        )
        .with_state(state)
        .layer(middleware::from_extractor::<AuthenticatedUser>());

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
