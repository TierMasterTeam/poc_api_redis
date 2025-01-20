use std::sync::Arc;
use redis::RedisResult;
use rocket::futures::lock::Mutex;
use crate::entities::user_entity::UserEntity;
use crate::repositories::r#impl::user_repository_impl::UserRepositoryImpl;

pub trait UserRepository: Send + Sync {
    fn get_user(&self, id: u64) -> RedisResult<UserEntity>;
    fn create_user(&self, user: &UserEntity) -> RedisResult<UserEntity>;
}

pub fn make_user_repository() -> Arc<dyn UserRepository> {
    Arc::new(UserRepositoryImpl::new())
}

