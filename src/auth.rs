use actix_web::dev::ServiceRequest;
use actix_web::Error;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &[u8] = b"your_secret_key"; // Replace this with a secure key

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID or username
    exp: usize,  // Expiration time (in seconds since Unix Epoch)
}

impl Claims {
    fn new(sub: String, expiration_in_minutes: i64) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(expiration_in_minutes))
            .expect("valid timestamp")
            .timestamp() as usize;

        Claims {
            sub,
            exp: expiration,
        }
    }
}

// Function to create a JWT token
pub fn create_jwt(sub: String) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims::new(sub, 60); // Token valid for 60 minutes
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    )
}

// Function to validate a JWT token
pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

// Middleware to verify JWT token
pub async fn jwt_middleware(req: ServiceRequest) -> Result<ServiceRequest, Error> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Extract token after "Bearer "
                match validate_jwt(token) {
                    Ok(_claims) => {
                        return Ok(req);
                    }
                    Err(_) => {
                        return Err(actix_web::error::ErrorUnauthorized("Invalid token"));
                    }
                }
            }
        }
    }

    Err(actix_web::error::ErrorUnauthorized(
        "Authorization header missing or malformed",
    ))
}
