use babestu_api;

#[rocket::launch]
fn rocket() -> rocket::Rocket {
    babestu_api::rocket()
}
