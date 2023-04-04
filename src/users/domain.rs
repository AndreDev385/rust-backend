use regex::Regex;

#[derive(Debug)]
pub struct User {
    pub email: Email,
    pub password: Password,
}

#[derive(Debug)]
pub struct Password {
    value: String,
}

impl Password {
    fn is_valid(value: &str) -> bool {
        value.len() > 8
    }

    pub fn create(value: &str) -> std::result::Result<Self, String> {
        if !Password::is_valid(value) {
            return Err(String::from("Invalid password"));
        }
        Ok(Password { value: String::from(value) })
    }

    pub fn value(self) -> String {
        self.value
    }
}

#[derive(Debug)]
pub struct Email {
    value: String,
}

impl Email {
    fn is_valid(value: &str) -> bool {
        let re = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();
        re.is_match(value)
    }

    pub fn create(value: &str) -> std::result::Result<Email, String> {
        if !Email::is_valid(value) {
            return Err(String::from("Invalid Email"));
        }
        Ok(Email {
            value: String::from(value),
        })
    }

    pub fn value(self) -> String {
        self.value
    }
}
