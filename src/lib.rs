#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::rusqlite;

#[database("babestu")]
struct Connection(rusqlite::Connection);

#[get("/")]
fn index() -> &'static str {
    "Welcome to babestu!"
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Connection::fairing())
        .mount("/", routes![index])
}
