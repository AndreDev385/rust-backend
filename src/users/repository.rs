use super::domain::User;

pub trait UserRepository {
    fn add_user(&mut self, user: User) -> ();

    fn list_users(&self) -> &Vec<User>;
}

#[derive(Debug)]
pub struct MemoryUserRepository {
    users_list: Vec<User>,
}

impl UserRepository for MemoryUserRepository {
    fn add_user(&mut self, user: User) -> () {
        self.users_list.push(user)
    }

    fn list_users(&self) -> &Vec<User> {
        &self.users_list
    }
}

impl MemoryUserRepository {
    pub fn create() -> Self {
        MemoryUserRepository { users_list: vec![] }
    }
}
