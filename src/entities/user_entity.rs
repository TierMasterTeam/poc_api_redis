use redis::{ErrorKind, FromRedisValue, RedisError, RedisResult, RedisWrite, ToRedisArgs, Value};
use rocket::serde::json::serde_json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserEntity {
    pub id: u64,
    pub name: String,
    pub email: String,
}