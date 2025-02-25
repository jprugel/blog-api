pub mod schema;
pub mod models;

use diesel::PgConnection;
use diesel::prelude::*;
use models::{NewPost, Post, PostUpdate};
use axum::{
    extract::{
        Path,
        Query
    },
    Json
};
use dotenvy::dotenv;
use std::env;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<usize>,
    per_page: Option<usize>,
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost::builder()
        .title(title.to_string())
        .body(body.to_string())
        .build();

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn get_posts(query: Query<Pagination>) -> Json<Vec<Post>> {
    use self::schema::posts::dsl::posts;
    let pagination = query.0;

    let other_posts = posts
        .select(Post::as_select())
        .limit(pagination.per_page.unwrap_or(999) as i64)
        .offset(((pagination.page.unwrap_or(1) - 1) * pagination.per_page.unwrap_or(999)) as i64)
        .load(&mut establish_connection())
        .expect("Failed  to get posts");

    Json(other_posts)
}

pub async fn get_post(Path(post_id) : Path<i32>) -> Json<Post> {
    use self::schema::posts::dsl::posts;

    let optional_post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(&mut establish_connection())
        .optional()
        .expect("Failed to get post");

    match optional_post {
        Some(post) => Json(post),
        None => Json::default()
    }
}

pub async fn add_post(Json(payload) : Json<NewPost>) {
    use crate::schema::posts;

    diesel::insert_into(posts::table)
        .values(&payload)
        .returning(Post::as_returning())
        .get_result(&mut establish_connection())
        .expect("Error saving new post");
}

pub async fn update_post(Path(post_id) : Path<i32>, Json(payload) : Json<PostUpdate>) {
    use crate::schema::posts::dsl::posts;

    let _ = diesel::update(posts.find(post_id))
        .set(payload)
        .execute(&mut establish_connection());
}

pub async fn delete_post(Path(post_id) : Path<i32>) {
    use crate::schema::posts::dsl::posts;

    let _ = diesel::delete(posts.find(post_id))
        .execute(&mut establish_connection());
}

