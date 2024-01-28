use crate::models::user_model::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {}
    }

    pub fn get_user(&self, id: i32) -> Option<User> {
        // ここでデータベースや外部APIからデータを取得します
        Some(User::new(id, String::from("sample")))
    }
}
