use crate::db::Pool;
use crate::handlers::{handle_create, handle_delete, handle_get, handle_update};
use warp::Filter;

fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn user_routes(
    pool: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // POST /users with JSON body
    let create_user = warp::post()
        .and(warp::path("adduser"))
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handle_create);

    // GET /users/:id
    let get_user = warp::get()
        .and(warp::path("getuser"))
        .and(warp::path::param::<i32>())
        .and(with_db(pool.clone()))
        .and_then(handle_get);

    // PUT /users/:id with JSON body
    let update_user = warp::put()
        .and(warp::path("updateuser"))
        .and(warp::path::param::<i32>())
        .and(warp::body::json())
        .and(with_db(pool.clone()))
        .and_then(handle_update);

    // DELETE /users/:id
    let delete_user = warp::delete()
        .and(warp::path("deleteuser"))
        .and(warp::path::param::<i32>())
        .and(with_db(pool.clone()))
        .and_then(handle_delete);

    create_user
        .or(get_user)
        .or(update_user)
        .or(delete_user)
        .boxed()
}
