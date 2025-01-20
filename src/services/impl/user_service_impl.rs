use std::sync::Arc;
use rocket::futures::lock::Mutex;
use crate::dto::user_dto::UserDto;
use crate::mappers::user_mapper::{dto_to_entity, entity_to_dto};
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

impl UserService for UserServiceImpl {
    fn get_user(&self, id: u64) -> UserDto {
        let user_entity = self.repository.get_user(id).unwrap();
        entity_to_dto(user_entity)
    }

    fn create_user(&self, user: UserDto) -> UserDto {
        let user_entity = dto_to_entity(user);
        let user_entity = self.repository.create_user(&user_entity).unwrap();
        entity_to_dto(user_entity)
    }
}