use crate::models::user_model::User;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository {}
    }

    pub fn get_user(&self, id: i32) -> Result<User, String> {
        todo!();
    }

    pub fn create_user(&self, user: User) -> Result<User, String> {
        todo!();
    }
}
