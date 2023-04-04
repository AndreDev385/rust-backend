
use super::super::domain::{Email, Password, User};
use super::super::dto::CreateUserDto;
use super::super::repository::UserRepository;
use std::result::Result;

pub fn add_user(user: CreateUserDto, repository: &mut dyn UserRepository) -> Result<(), String> {
    let email = Email::create(&user.email);
    let password = Password::create(&user.password);

    if email.is_err() {
        return Err(format!("{}", email.unwrap_err()));
    }

    if password.is_err() {
        return Err(format!("{}", password.unwrap_err()));
    }

    let user = User {
        email: email.unwrap(),
        password: password.unwrap(),
    };

    repository.add_user(user);

    Ok(())
}
