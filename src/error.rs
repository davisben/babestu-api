use rocket::Catcher;
use rocket_contrib::json::JsonValue;

pub fn catchers() -> Vec<Catcher> {
    catchers![
        not_found,
        unprocessable_entity,
        server_error
     ]
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": 404,
        "message": "Not found"
    })
}

#[catch(422)]
fn unprocessable_entity() -> JsonValue {
    json!({
        "status": 422,
        "message": "Unprocessable Entity"
    })
}

#[catch(500)]
fn server_error() -> JsonValue {
    json!({
        "status": 500,
        "message": "Internal Server Error"
    })
}
