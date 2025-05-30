use crate::models::{NewUser, UpdateUser, User};
use crate::schema::user::dsl::*;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(database_url: &str) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn create_user(conn: &mut MysqlConnection, new_user: NewUser) -> QueryResult<User> {
    diesel::insert_into(user).values(&new_user).execute(conn)?;

    user.order(id.desc()).first(conn)
}

pub fn get_user(conn: &mut MysqlConnection, user_id: i32) -> QueryResult<User> {
    user.find(user_id).first(conn)
}

pub fn update_user(
    conn: &mut MysqlConnection,
    user_id: i32,
    updated: UpdateUser,
) -> QueryResult<User> {
    diesel::update(user.find(user_id))
        .set(updated)
        .execute(conn)?;

    user.find(user_id).first(conn)
}

pub fn delete_user(conn: &mut MysqlConnection, user_id: i32) -> QueryResult<usize> {
    diesel::delete(user.find(user_id)).execute(conn)
}
