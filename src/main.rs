mod models;
mod db;

use warp::reply::Reply;
use warp::{filters::method::post, Filter};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use models::*;
use db::Db;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::convert::Infallible;
use serde::de::DeserializeOwned;

#[tokio::main]
async fn main() {


    let db: Db = Arc::new(Mutex::new(HashMap::new()));
    println!("Hello, world!");

    let register = warp::path("register")
    .and(warp::post())
    .and(json_body())
    .and(with_db(db.clone()))
    .and_then(register_user);

    let login = warp::path("login")
    .and(post())
    .and(json_body())
    .and(with_db(db.clone()))
    .and_then(login_user);

    let get_profile = warp::path("profile")
    .and(warp::path::param::<Uuid>())
    .and(warp::get())
    .and(with_db(db.clone()))
    .and_then(get_user);

    let update_profile = warp::path("profile")
    .and(warp::path::param::<Uuid>())
    .and(warp::put())
    .and(json_body())
    .and(with_db(db.clone()))
    .and_then(update_user);

    let delete_profile = warp::path("profile")
    .and(warp::path::param::<Uuid>())
    .and(warp::delete())
    .and(with_db(db.clone()))
    .and_then(delete_user);

    let routes = register
    .or(login)
    .or(get_profile)
    .or(update_profile)
    .or(delete_profile);

    println!("Server running @ http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

//cloning the database and passing it into the function
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

//converting JSON body into a struct
fn json_body<T: DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

async fn register_user(body: RegisterRequest, db: Db) -> Result<warp::reply::Response, warp::Rejection> {
    let user_id = Uuid::new_v4();

    let hashed_password = hash(&body.password, DEFAULT_COST).unwrap();

    let name = body.name.clone();
    let user = User {
        id: user_id,
        name: body.name,
        email: body.email,
        password: hashed_password,
    };

    db.lock().unwrap().insert(user_id, user);

    Ok(warp::reply::json(&serde_json::json!({
        "id": user_id,
        "name": name
    })).into_response())
}

async fn login_user(body: LoginRequest, db: Db) -> Result<warp::reply::Response, warp::Rejection> {
    let users = db.lock().unwrap();

    // Find a user with the given email
    for user in users.values() {
        if user.email == body.email {
            // Check if password matches
            if verify(&body.password, &user.password).unwrap() {
                return Ok(warp::reply::json(&serde_json::json!({
                    "message": "Login successful",
                    "user_id": user.id
                })).into_response());
            } else {
                break;
            }
        }
    }

    // If we reach here, login failed
    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "error": "Invalid email or password" })),
        warp::http::StatusCode::UNAUTHORIZED,
    ).into_response())
}

async fn update_user(user_id: Uuid, body: UpdateRequest, db: Db) -> Result<warp::reply::Response, warp::Rejection> {
    let mut users = db.lock().unwrap();

    if let Some(user) = users.get_mut(&user_id) {

        if let Some(name) = body.name.clone() {
            user.name = name;
        }


        if let Some(password) = body.password {
            user.password = hash(&password, DEFAULT_COST).unwrap();
        }

        return Ok(warp::reply::json(&serde_json::json!({
            "message": "User updated"
        })).into_response());
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "error": "User not found" })),
        warp::http::StatusCode::NOT_FOUND,
    ).into_response())
}


async fn get_user(user_id: Uuid, db: Db) -> Result<warp::reply::Response, warp::Rejection> {
    let users = db.lock().unwrap();

    if let Some(user) = users.get(&user_id) {
        return Ok(warp::reply::json(&serde_json::json!({
            "id": user.id,
            "name": user.name,
            "email": user.email
        })).into_response());
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "error": "User not found" })),
        warp::http::StatusCode::NOT_FOUND,
    ).into_response())
}


async fn delete_user(user_id: Uuid, db: Db) -> Result<warp::reply::Response, warp::Rejection> {
    let mut users = db.lock().unwrap();

    if users.remove(&user_id).is_some() {
        return Ok(warp::reply::json(&serde_json::json!({
            "message": "User deleted"
        })).into_response());
    }

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({ "error": "User not found" })),
        warp::http::StatusCode::NOT_FOUND,
    ).into_response())
}
