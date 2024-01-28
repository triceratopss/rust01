use rocket::*;

mod handlers;
mod models;
mod repositories;
mod services;

use crate::handlers::user_handler::*;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let user_repository = UserRepository::new();
    let user_service = UserService::new(user_repository);

    rocket::build()
        .manage(user_service)
        .mount("/", routes![index, get_user])
}
