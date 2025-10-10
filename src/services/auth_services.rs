use argon2::{self, Config};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;

/// Hash user password using Argon2
pub fn hash_password(password: &str) -> String {
    let salt = b"random_salt";
    argon2::hash_encoded(password.as_bytes(), salt, &Config::default()).unwrap()
}

/// Verify password
pub fn verify_password(password: &str, hashed: &str) -> bool {
    argon2::verify_encoded(hashed, password.as_bytes()).unwrap_or(false)
}

/// JWT claims
#[derive(Serialize)]
struct Claims {
    pub sub: i32,
    pub exp: usize,
}

/// Generate JWT token
pub fn generate_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id,
        exp: (chrono::Utc::now().timestamp() + 60 * 60) as usize, // 1 hr expiration
    };

    let secret = std::env::var("JWT_SECRET").unwrap_or("secret".to_string());
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}
