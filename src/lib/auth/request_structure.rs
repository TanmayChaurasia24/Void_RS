use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegisterRequestStructure {
    pub full_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequestStructure {
    pub email: String,
    pub password: String,
}