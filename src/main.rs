#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to babestu!"
}

#[rocket::launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}
