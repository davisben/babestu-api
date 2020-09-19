use diesel;
use diesel::prelude::*;
use super::DbConn;
use super::schema::users;
use model::{User, NewUser, UpdatedUser};

pub mod routes;
mod model;

pub fn insert(user: NewUser, conn: DbConn) -> QueryResult<User> {
    diesel::insert_into(users::table)
        .values(user)
        .execute(&*conn)?;

    users::table.order(users::id.desc()).first(&*conn)
}

pub fn all(conn: DbConn) -> QueryResult<Vec<User>> {
    users::table.load(&*conn)
}

pub fn get(id: i32, conn: DbConn) -> QueryResult<User> {
    users::table.find(id).get_result(&*conn)
}

pub fn update(id: i32, user: UpdatedUser, conn: DbConn) -> QueryResult<User> {
    diesel::update(users::table.find(id))
        .set(user)
        .execute(&*conn)?;

    users::table.find(id).get_result(&*conn)
}

pub fn delete(id: i32, conn: DbConn) -> QueryResult<usize> {
    diesel::delete(users::table.find(id))
        .execute(&*conn)
}
