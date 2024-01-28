use crate::models::user_model::User;
use crate::repositories::user_repository::UserRepository;

pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        UserService { user_repository }
    }

    pub fn get_user(&self, id: i32) -> Option<User> {
        self.user_repository.get_user(id)
    }
}
