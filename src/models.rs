use diesel::prelude::*;
use serde::{
    Deserialize,
    Serialize
};
use bon::Builder;

#[derive(Queryable, Selectable, Deserialize, Debug, Default, Serialize)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Builder, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(Builder, AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::posts)]
pub struct PostUpdate {
    title: Option<String>,
    body: Option<String>
}
