use redis::{Commands, ErrorKind, RedisError, RedisResult, Value};
use crate::entities::user_entity::UserEntity;
use crate::repositories::redis_connector::connect_to_redis;
use crate::repositories::user_repository::UserRepository;

pub struct UserRepositoryImpl {
    
}

impl UserRepositoryImpl {
    pub fn new() -> Self {
        UserRepositoryImpl{}
    }
}

fn make_user_key(id: u64) -> String {
    format!("user:{}", id)
}

impl UserRepository for UserRepositoryImpl {
    fn get_user(&self, id: u64) -> RedisResult<UserEntity> {
        let mut redis = connect_to_redis()?;
        let key = make_user_key(id);
        let json_data: Vec<u8> = redis.get(key)?;
        let user_entity: UserEntity = serde_json::from_slice(&json_data)
            .map_err(|e| redis::RedisError::from((redis::ErrorKind::TypeError, "Failed to parse user entity")))?;
        Ok(user_entity)
    }

    fn create_user(&self, user: &UserEntity) -> RedisResult<UserEntity> {
        let mut redis = connect_to_redis()?;
        let key = make_user_key(user.id);

        let json_data = serde_json::to_vec(user).unwrap();
        let data: Value = redis.set(key, json_data)?;
        self.get_user(user.id)
    }
}