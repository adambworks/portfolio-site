use actix_web::{web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};
use crate::db::{users, DBPool};
use crate::models::{NewUser, User};
use chrono::{Utc, Duration};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// In a real application, this should be loaded from a secure configuration
pub const JWT_SECRET: &[u8] = b"your-super-secret-key";

pub async fn register(
    pool: web::Data<DBPool>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let hashed_password = match hash(&req.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let username = req.into_inner().username;

    match web::block(move || {
        let new_user = NewUser {
            username: &username,
            password_hash: &hashed_password,
        };
        users::create_user(&mut conn, &new_user)
    })
    .await
    {
        Ok(Ok(_)) => HttpResponse::Ok().json("User created successfully"),
        _ => HttpResponse::InternalServerError().json("Error creating user"),
    }
}

pub async fn login(
    pool: web::Data<DBPool>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let LoginRequest { username, password } = req.into_inner();

    let user = match web::block(move || users::find_user_by_username(&mut conn, &username)).await {
        Ok(Ok(user)) => user,
        _ => return HttpResponse::Unauthorized().json("Invalid username or password"),
    };

    match verify(&password, &user.password_hash) {
        Ok(true) => {
            let exp = (Utc::now() + Duration::days(1)).timestamp() as usize;
            let claims = Claims {
                sub: user.username.clone(),
                exp,
            };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap();
            HttpResponse::Ok().json(TokenResponse { token })
        }
        _ => HttpResponse::Unauthorized().json("Invalid username or password"),
    }
}