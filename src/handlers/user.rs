use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use sea_orm::*;

use super::{ErrorResponse, Response, SuccessResponse};

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

#[put("/<id>", data = "<req_update_user>")]
pub async fn update_user(
    db: &State<DatabaseConnection>,
    id: u32,
    req_update_user: Json<ReqCreateUpdateUser>,
) -> Response<Json<ResUser>> {
    let db = db as &DatabaseConnection;

    let mut user: users::ActiveModel = match Users::find_by_id(id).one(db).await? {
        Some(a) => a.into(),
        None => {
            return Err(ErrorResponse((
                Status::NotFound,
                "No user with the specified ID.".to_string(),
            )))
        }
    };

    user.name = Set(req_update_user.name.to_owned());
    user.age = Set(req_update_user.age as i64);

    user.updated_at = Set(DateTime::<Utc>::from(
        UNIX_EPOCH
            + SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards"),
    )
    .naive_utc());

    let user = user.update(db).await?;

    Ok(SuccessResponse((
        Status::Created,
        Json(ResUser {
            id: user.id as i32,
            name: user.name,
            age: user.age as i32,
        }),
    )))
}

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

#[get("/<id>")]
pub async fn get_user_one(db: &State<DatabaseConnection>, id: u32) -> Response<Json<ResUser>> {
    let db = db as &DatabaseConnection;

    let user = Users::find_by_id(id).one(db).await?;

    let user = match user {
        Some(u) => u,
        None => {
            return Err(super::ErrorResponse((
                Status::NotFound,
                "Cannot find a user with the specified ID.".to_string(),
            )))
        }
    };

    Ok(SuccessResponse((
        Status::Ok,
        Json(ResUser {
            id: user.id as i32,
            name: user.name,
            age: user.age as i32,
        }),
    )))
}
