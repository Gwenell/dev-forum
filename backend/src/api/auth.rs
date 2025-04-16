use crate::auth::{self, jwt, password, AuthPayload};
use crate::db::DbPool;
use crate::error::AppError;
use crate::models::user::{self, ActiveModel as UserActiveModel};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub username: String,
}

async fn login(
    pool: web::Data<DatabaseConnection>,
    web::Json(req): web::Json<LoginRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    // Find the user by username
    let user = auth::find_user_by_username(pool.get_ref(), &req.username)
        .await?
        .ok_or_else(|| AppError::authentication_error("Invalid username or password"))?;

    // Verify the password
    let is_valid = password::verify_password(&req.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::authentication_error("Invalid username or password"));
    }

    // Update last login time
    let mut user_model: UserActiveModel = user.clone().into();
    user_model.last_login = Set(Some(Utc::now()));
    user_model.update(pool.get_ref()).await?;

    // Create an auth payload and generate a JWT token
    let payload = AuthPayload {
        user_id: user.id,
        username: user.username.clone(),
        is_admin: user.is_admin,
    };

    let token = jwt::create_token(&payload)?;

    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user_id: payload.user_id.to_string(),
        username: payload.username,
        is_admin: payload.is_admin,
    }))
}

async fn register(
    pool: web::Data<DatabaseConnection>,
    web::Json(req): web::Json<RegisterRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    // Check if username already exists
    if let Some(_) = auth::find_user_by_username(pool.get_ref(), &req.username).await? {
        return Err(AppError::ValidationError("Username already taken".to_string()));
    }

    // Check if email already exists
    if let Some(_) = auth::find_user_by_email(pool.get_ref(), &req.email).await? {
        return Err(AppError::ValidationError("Email already registered".to_string()));
    }

    // Hash the password
    let password_hash = password::hash_password(&req.password)?;

    // Create a new user
    let user_id = Uuid::new_v4();
    let now = Utc::now();
    let user = UserActiveModel {
        id: Set(user_id),
        username: Set(req.username.clone()),
        email: Set(req.email),
        password_hash: Set(password_hash),
        display_name: Set(None),
        bio: Set(None),
        avatar_url: Set(None),
        theme_preference: Set(None),
        is_admin: Set(false),
        is_active: Set(true),
        created_at: Set(now),
        updated_at: Set(now),
        last_login: Set(Some(now)),
    };

    // Insert the user into the database
    let user = user.insert(pool.get_ref()).await?;

    Ok(HttpResponse::Created().json(RegisterResponse {
        user_id: user.id.to_string(),
        username: user.username,
    }))
}

async fn validate_token(
    auth_payload: web::ReqData<AuthPayload>,
) -> Result<impl Responder, AppError> {
    Ok(HttpResponse::Ok().json(auth_payload.into_inner()))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/register", web::post().to(register))
            .route("/validate", web::get().to(validate_token)),
    );
} 