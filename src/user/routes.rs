use rocket::Route;
use rocket::http::Status;
use rocket_contrib::json::Json;
use super::DbConn;
use super::model::{User, UserData};

pub fn routes() -> Vec<Route> {
    routes![insert, all, get, update, delete]
}

#[post("/", format = "json", data = "<data>")]
fn insert(data: Json<UserData>, conn: DbConn) -> Result<Json<User>, Status> {
    let user = data.into_inner().to_new_user();

    super::insert(user, conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
pub fn all(conn: DbConn) -> Result<Json<Vec<User>>, Status> {
    super::all(conn)
        .map(|users| Json(users))
        .map_err(|_| Status::InternalServerError)
}

#[get("/<id>")]
pub fn get(id: i32, conn: DbConn) -> Result<Json<User>, Status> {
    super::get(id, conn)
        .map(|user| Json(user))
        .map_err(|_| Status::InternalServerError)
}

#[put("/<id>", format = "json", data = "<data>")]
fn update(id: i32, data: Json<UserData>, conn: DbConn) -> Result<Json<User>, Status> {
    let user = data.into_inner().to_updated_user();

    super::update(id, user, conn)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[delete("/<id>")]
fn delete(id: i32, conn: DbConn) -> Result<Status, Status> {
    super::delete(id, conn)
        .map(|_| Status::NoContent)
        .map_err(|_| Status::InternalServerError)
}
