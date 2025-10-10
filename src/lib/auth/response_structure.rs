use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RegisterResponseStructure {
    pub user_id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct LoginResponseStructure {
    pub user_id: String,
    pub token: String,
    pub message: String,
}