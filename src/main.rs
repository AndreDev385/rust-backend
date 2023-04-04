use tide::http::mime::JSON;
use tide::prelude::json;
use tide::{Request, Response, Result};

use users::dto::CreateUserDto;
use users::repository::MemoryUserRepository;
use users::use_cases::add_user::add_user;

use crate::users::repository::UserRepository;

pub mod users;

async fn check_health(_req: Request<()>) -> Result<String> {
    Ok(String::from("Everything is ok"))
}

async fn create_user(mut req: Request<()>) -> Result<Response> {
    let data: CreateUserDto = req.body_json().await?;

    let mut memory_repository = MemoryUserRepository::create();

    println!("{:#?}", memory_repository.list_users());

    match add_user(data, &mut memory_repository) {
        Ok(()) => Ok(Response::builder(201)
            .body(json!({"message": "User has been created"}))
            .content_type(JSON)
            .build()),
        Err(e) => Ok(Response::builder(400)
            .body(json!({ "error": e }))
            .content_type(JSON)
            .build()),
    }
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/check-health").get(check_health);
    app.at("/api/users").post(create_user);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
