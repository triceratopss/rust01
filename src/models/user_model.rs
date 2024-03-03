use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: i32,
    name: String,
    age: i32,
}

impl User {
    pub fn new(id: i32, name: String, age: i32) -> Self {
        User { id, name, age }
    }
}
