use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

// impl NewUser {
//     pub fn new(name: String, email: String, password: String) -> NewUser {
//         let hashed_password: String = hash(password.as_str(), DEFAULT_COST).unwrap();
//         let uuid = Uuid::new_v4().to_string();
//         return NewUser {name, email, password: hashed_password, unique_id: uuid}
//     }
// }