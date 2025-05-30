use crate::db::{Pool, create_user, delete_user, get_user, update_user};
use crate::models::{NewUser, UpdateUser};
use serde_json::json;
use warp::http::StatusCode;
use warp::reply::{Json, json};
use warp::{Rejection, Reply};

#[derive(Debug)]
struct InternalServerError;

impl warp::reject::Reject for InternalServerError {}

pub async fn handle_create(new: NewUser, pool: Pool) -> Result<Json, Rejection> {
    let mut conn = pool
        .get()
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    let created = tokio::task::spawn_blocking(move || create_user(&mut conn, new))
        .await
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    match created {
        Ok(user) => Ok(json(&json!({
            "status": "success",
            "message": "User created successfully",
            "data": user
        }))),
        Err(_) => Err(warp::reject::custom(InternalServerError)),
    }
}

pub async fn handle_get(user_id: i32, pool: Pool) -> Result<Json, Rejection> {
    let mut conn = pool
        .get()
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    let result = tokio::task::spawn_blocking(move || get_user(&mut conn, user_id))
        .await
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    match result {
        Ok(user) => Ok(json(&json!({
            "status": "success",
            "message": "User retrieved successfully",
            "data": user
        }))),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn handle_update(
    user_id: i32,
    updated: UpdateUser,
    pool: Pool,
) -> Result<Json, Rejection> {
    let mut conn = pool
        .get()
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    let result = tokio::task::spawn_blocking(move || update_user(&mut conn, user_id, updated))
        .await
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    match result {
        Ok(user) => Ok(json(&json!({
            "status": "success",
            "message": "User updated successfully",
            "data": user
        }))),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn handle_delete(user_id: i32, pool: Pool) -> Result<impl Reply, Rejection> {
    let mut conn = pool
        .get()
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    let result = tokio::task::spawn_blocking(move || delete_user(&mut conn, user_id))
        .await
        .map_err(|_| warp::reject::custom(InternalServerError))?;

    match result {
        Ok(_) => Ok(warp::reply::with_status(
            json(&json!({
                "status": "success",
                "message": format!("User with id {} deleted successfully", user_id)
            })),
            StatusCode::OK,
        )),
        Err(_) => Err(warp::reject::not_found()),
    }
}
