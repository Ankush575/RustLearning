/* mod stands for module here we are importing the modules/files that will be used */
mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod schema;

/* importing of specific modu*/
use crate::routes::user_routes;
use db::establish_connection;
use dotenv::dotenv;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_connection(&database_url);

    let routes = user_routes(pool).with(warp::cors().allow_any_origin());

    println!("Server running at http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
