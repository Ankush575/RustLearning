use crate::schema::user;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(diesel::Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub gender: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(diesel::Insertable, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub age: i32,
    pub gender: String,
}

#[derive(diesel::AsChangeset, Deserialize)]
#[diesel(table_name = user)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub gender: Option<String>,
}
