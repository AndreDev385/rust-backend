use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUserDto {
    pub email: String,
    pub password: String,
}
