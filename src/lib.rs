pub mod auth;
pub mod models;
pub mod schema;

use crate::auth::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use bon::Builder;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use models::{NewPost, Post, PostUpdate};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

#[derive(Deserialize)]
pub struct Search {
    query: Option<String>,
}

#[derive(Clone, Builder)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Failed to build connection pool.")
}

pub async fn get_posts(
    State(state): State<AppState>,
    query: Query<Pagination>,
    search_query: Query<Search>,
) -> Json<Vec<Post>> {
    use self::schema::posts::dsl::{body, posts};
    let pagination = query.0;
    let search = match search_query.0.query {
        Some(s) => s,
        None => "".to_string(),
    };

    let other_posts = posts
        .select(Post::as_select())
        .limit(pagination.per_page.unwrap_or(999) as i64)
        .offset(((pagination.page.unwrap_or(1) - 1) * pagination.per_page.unwrap_or(999)) as i64)
        .filter(body.ilike(format!("%{}%", search)))
        .load(&mut state.pool.get().unwrap())
        .expect("Failed  to get posts");

    Json(other_posts)
}

pub async fn get_post(State(state): State<AppState>, Path(post_id): Path<i32>) -> Json<Post> {
    use self::schema::posts::dsl::posts;

    let optional_post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(&mut state.pool.get().unwrap())
        .optional()
        .expect("Failed to get post");

    match optional_post {
        Some(post) => Json(post),
        None => Json::default(),
    }
}

pub async fn add_post(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<NewPost>,
) -> Result<StatusCode, (StatusCode, &'static str)> {
    use crate::schema::posts;
    if !user.roles.contains(&Role::Writer) && !user.roles.contains(&Role::Admin) {
        return Err((StatusCode::UNAUTHORIZED, "Insufficient authority"));
    }

    diesel::insert_into(posts::table)
        .values(&payload)
        .returning(Post::as_returning())
        .get_result(&mut state.pool.get().unwrap())
        .expect("Error saving new post");

    Ok(StatusCode::ACCEPTED)
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(post_id): Path<i32>,
    Json(payload): Json<PostUpdate>,
) {
    use crate::schema::posts::dsl::posts;

    let _ = diesel::update(posts.find(post_id))
        .set(payload)
        .execute(&mut state.pool.get().unwrap());
}

pub async fn delete_post(State(state): State<AppState>, Path(post_id): Path<i32>) {
    use crate::schema::posts::dsl::posts;

    let _ = diesel::delete(posts.find(post_id)).execute(&mut state.pool.get().unwrap());
}
