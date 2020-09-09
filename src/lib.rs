#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::databases::rusqlite;

#[get("/")]
fn index() -> &'static str {
    "Welcome to babestu!"
}

pub fn rocket() -> rocket::Rocket {
    #[database("babestu")]
    struct DbConn(rusqlite::Connection);

    run_migrations();

    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes())
}

pub fn rocket_testing() -> rocket::Rocket {
    #[database("testing")]
    struct DbConn(rusqlite::Connection);

    run_migrations();

    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/", routes())
}

fn routes() -> Vec<rocket::Route> {
    routes![index]
}

fn run_migrations() {
    let mut conn = rusqlite::Connection::open_in_memory().unwrap();
    embedded::migrations::runner().run(&mut conn).unwrap();
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}
