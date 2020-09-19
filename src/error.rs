use rocket::Catcher;
use rocket::Request;
use rocket::response::{Responder, Response, Result};
use rocket::http::{ContentType, Status};
use serde::Serialize;

pub struct ErrorResponse {
    pub status: Status,
    pub message: String,
}

#[derive(Serialize)]
struct ErrorResponseData {
    status: String,
    message: String,
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, request: &'r Request<'_>) -> Result<'static> {
        let response_data = ErrorResponseData {
            status: String::from("error"),
            message: self.message
        };
        let json = json!(response_data);

        Response::build_from(json.respond_to(&request).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

pub fn catchers() -> Vec<Catcher> {
    catchers![
        not_found,
        bad_request,
        unprocessable_entity,
        server_error
     ]
}

#[catch(404)]
fn not_found() -> ErrorResponse {
    ErrorResponse {
        status: Status::NotFound,
        message: String::from("Not found")
    }
}

#[catch(400)]
fn bad_request() -> ErrorResponse {
    ErrorResponse {
        status: Status::BadRequest,
        message: String::from("Bad request")
    }
}

#[catch(422)]
fn unprocessable_entity() -> ErrorResponse {
    ErrorResponse {
        status: Status::UnprocessableEntity,
        message: String::from("Unprocessable entity")
    }
}

#[catch(500)]
fn server_error() -> ErrorResponse {
    ErrorResponse {
        status: Status::InternalServerError,
        message: String::from("Internal server error")
    }
}
