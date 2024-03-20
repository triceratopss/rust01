#[macro_use]
extern crate rocket;

mod db;
mod entities;
mod fairings;
mod handlers;

use fairings::cors::{options, CORS};
use handlers::{Response, SuccessResponse};
use rocket::http::Status;

pub struct AppConfig {
    db_host: String,
    db_port: String,
    db_user: String,
    db_password: String,
    db_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            db_host: std::env::var("DB_HOST").unwrap_or("localhost".to_string()),
            db_port: std::env::var("DB_PORT").unwrap_or("15433".to_string()),
            db_user: std::env::var("DB_USER").unwrap_or("rust01".to_string()),
            db_password: std::env::var("DB_PASSWORD").unwrap_or("password001".to_string()),
            db_name: std::env::var("DB_NAME").unwrap_or("rust01".to_string()),
        }
    }
}

#[get("/")]
fn index() -> Response<String> {
    Ok(SuccessResponse((Status::Ok, "Hello, world!".to_string())))
}

#[launch]
async fn rocket() -> _ {
    let config = AppConfig::default();

    let db = db::connect(&config).await.unwrap();
    rocket::build()
        .attach(CORS)
        .manage(db)
        .mount("/", routes![options])
        .mount("/", routes![index])
        .mount(
            "/users",
            routes![
                handlers::user::get_user_one,
                handlers::user::get_user_list,
                handlers::user::create_user,
                handlers::user::update_user,
                handlers::user::delete_user
            ],
        )
}
