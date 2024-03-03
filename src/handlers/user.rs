use super::Response;

#[post("/")]
pub async fn create_user() -> Response<String> {
    todo!()
}

#[put("/<id>")]
pub async fn update_user(id: u32) -> Response<String> {
    todo!()
}

#[delete("/<id>")]
pub async fn delete_user(id: u32) -> Response<String> {
    todo!()
}

#[get("/")]
pub async fn get_user_list() -> Response<String> {
    todo!()
}

#[get("/<id>")]
pub async fn get_user_one(id: u32) -> Response<String> {
    todo!()
}
