use rocket::serde::{json::Json, Deserialize, Serialize};

use super::Response;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateUpdateUser {
    pub name: String,
    pub age: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResUser {
    id: i32,
    name: String,
    age: i32,
}

#[post("/", data = "<req_create_user>")]
pub async fn create_user(req_create_user: Json<ReqCreateUpdateUser>) -> Response<Json<ResUser>> {
    todo!()
}

#[put("/<id>", data = "<req_update_user>")]
pub async fn update_user(
    id: u32,
    req_update_user: Json<ReqCreateUpdateUser>,
) -> Response<Json<ResUser>> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete_user(id: u32) -> Response<String> {
    todo!()
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResGetUserList {
    total: usize,
    users: Vec<ResUser>,
}

#[get("/")]
pub async fn get_user_list() -> Response<Json<ResGetUserList>> {
    todo!()
}

#[get("/<id>")]
pub async fn get_user_one(id: u32) -> Response<Json<ResUser>> {
    todo!()
}
