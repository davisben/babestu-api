#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;

use rocket::Rocket;
use diesel::sqlite::SqliteConnection;

#[database("babestu")]
pub struct DbConn(SqliteConnection);

mod error;
mod schema;
mod user;

pub fn rocket() -> Rocket {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/user", user::routes::routes())
        .register(error::catchers())

}
