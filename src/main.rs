use axum::{
    Router,
    routing::get,
};

use blog_api::*;

const VERSION_ONE: &'static str = "/api/v1/";

#[tokio::main]
async fn main() {
   // build our application with a single route
    let app = Router::new()
        .route(&(VERSION_ONE.to_owned() + "blogs/{payload}"), get(get_post).patch(update_post).delete(delete_post))
        .route(&(VERSION_ONE.to_owned() + "blogs"), get(get_posts).post(add_post));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
