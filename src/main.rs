use rocket::{get, routes, launch};
use tiermaster_api_poc::controllers::user_controller::{create_user, get_all_users, get_user};
use tiermaster_api_poc::services::user_service::make_user_service;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {name}")
}

#[launch]
fn rocket() -> _ {
    let user_service = make_user_service();
    
    rocket::build()
        .manage(user_service)
        .mount("/", routes![index, hello, get_all_users, get_user, create_user])
}