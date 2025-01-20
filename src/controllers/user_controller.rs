use std::sync::Arc;
use rocket::{get, post, State};
use rocket::serde::json::Json;
use crate::dto::user_dto::UserDto;
use crate::services::user_service::UserService;

#[get("/users")]
pub fn get_all_users(user_service: &State<Arc<dyn UserService>>) -> Json<UserDto> {
    return Json(UserDto {
        id: 1,
        name: "John Smith".to_string(),
        email: "john@doe.com".to_string(),
    });
}

#[get("/users/<id>")]
pub fn get_user(user_service: &State<Arc<dyn UserService>>, id: u64) -> Json<UserDto> {
    return Json(user_service.get_user(id));
}

#[post("/users", format = "application/json", data = "<user>")]
pub fn create_user(user_service: &State<Arc<dyn UserService>>, user: Json<UserDto>) -> Json<UserDto> {
    return Json(user_service.create_user(user.into_inner()));
}
