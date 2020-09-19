use rocket::Route;
use rocket::http::Status;
use rocket_contrib::json::Json;
use crate::error::ErrorResponse;
use super::DbConn;
use super::model::{User, UserData};

pub fn routes() -> Vec<Route> {
    routes![insert, all, get, update, delete]
}

#[post("/", format = "json", data = "<data>")]
fn insert(data: Json<UserData>, conn: DbConn) -> Result<Json<User>, ErrorResponse> {
    let new_user = match data.into_inner().to_new_user() {
        Ok(new_user) => new_user,
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };

    let user = match super::insert(new_user, conn) {
        Ok(user) => user,
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };

    Ok(Json(user))
}

#[get("/")]
pub fn all(conn: DbConn) -> Result<Json<Vec<User>>, ErrorResponse> {
    let users = match super::all(conn) {
        Ok(users) => users,
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };

    Ok(Json(users))
}

#[get("/<id>")]
pub fn get(id: i32, conn: DbConn) -> Result<Json<User>, ErrorResponse> {
    let user = match super::get(id, conn) {
        Ok(user) => user,
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };

    Ok(Json(user))
}

#[put("/<id>", format = "json", data = "<data>")]
fn update(id: i32, data: Json<UserData>, conn: DbConn) -> Result<Json<User>, ErrorResponse> {
    let updated_user = match data.into_inner().to_updated_user() {
        Ok(updated_user) => updated_user,
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };

    let user = match super::update(id, updated_user, conn) {
        Ok(user) => user,
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };

    Ok(Json(user))
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Status, ErrorResponse> {
    match super::delete(id, conn) {
        Ok(_) => return Ok(Status::Ok),
        Err(e) => {
            let response =  ErrorResponse {
                status: Status::InternalServerError,
                message: e.to_string()
            };
            return Err(response);
        }
    };
}
