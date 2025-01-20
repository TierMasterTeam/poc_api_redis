use std::sync::Arc;
use redis::RedisResult;
use rocket::futures::lock::Mutex;
use crate::dto::user_dto::UserDto;
use crate::entities::user_entity::UserEntity;
use crate::repositories::user_repository::{make_user_repository, UserRepository};
use crate::services::r#impl::user_service_impl::UserServiceImpl;

pub trait UserService: Send + Sync {
    fn get_user(&self, id: u64) -> UserDto;
    fn create_user(&self, user: UserDto) -> UserDto;
}

pub fn make_user_service() -> Arc<dyn UserService>  {
    Arc::new(UserServiceImpl::new(make_user_repository()))
}