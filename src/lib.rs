#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket::Rocket;
use diesel::sqlite::SqliteConnection;

#[database("babestu")]
pub struct DbConn(SqliteConnection);

mod schema;
mod models;
mod routes;

pub fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/user", routes::user::routes())
        .attach(DbConn::fairing())
}
