use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::*;

use super::{Response, SuccessResponse};

use crate::entities::{prelude::*, users};

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
pub async fn create_user(
    db: &State<DatabaseConnection>,
    req_create_user: Json<ReqCreateUpdateUser>,
) -> Response<Json<ResUser>> {
    let db = db as &DatabaseConnection;

    let user = users::ActiveModel {
        name: Set(req_create_user.name.to_owned()),
        age: Set(req_create_user.age as i64),
        ..Default::default()
    };

    let user = user.insert(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResUser {
            id: user.id as i32,
            name: user.name,
            age: user.age as i32,
        }),
    )))
}

// #[put("/<id>", data = "<req_update_user>")]
// pub async fn update_user(
//     id: u32,
//     req_update_user: Json<ReqCreateUpdateUser>,
// ) -> Response<Json<ResUser>> {
//     todo!()
// }

// #[delete("/<id>")]
// pub async fn delete_user(id: u32) -> Response<String> {
//     todo!()
// }

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ResGetUserList {
    total: usize,
    users: Vec<ResUser>,
}

#[get("/")]
pub async fn get_user_list(db: &State<DatabaseConnection>) -> Response<Json<ResGetUserList>> {
    let db = db as &DatabaseConnection;

    let users = Users::find()
        .all(db)
        .await?
        .iter()
        .map(|user| ResUser {
            id: user.id as i32,
            name: user.name.to_owned(),
            age: user.age as i32,
        })
        .collect::<Vec<_>>();

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResGetUserList {
            total: users.len(),
            users,
        }),
    )))
}

// #[get("/<id>")]
// pub async fn get_user_one(id: u32) -> Response<Json<ResUser>> {
//     todo!()
// }
