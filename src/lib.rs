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
    run_migrations();

    rocket::ignite()
        .attach(Connection::fairing())
        .mount("/", routes![index])
}

fn run_migrations() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
