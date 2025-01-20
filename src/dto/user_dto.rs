use rocket::data::{FromData, Outcome, ToByteUnit};
use rocket::{serde, Data, Request};
use rocket::http::{ContentType, Status};
use rocket::tokio::io::AsyncReadExt;

const LIMIT: u64 = 256;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserDto {
    pub id: u64,
    pub name: String,
    pub email: String,
}