use crate::models::user_model::User;
use crate::services::user_service::UserService;
use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;

#[get("/users/<id>")]
pub fn get_user(user_service: &rocket::State<UserService>, id: i32) -> Result<Json<User>, Status> {
    match user_service.get_user(id) {
        Some(user) => Ok(Json(user)),
        None => Err(Status::NotFound),
    }
}
