use crate::auth::{self, password, AuthPayload};
use crate::db::DbPool;
use crate::error::AppError;
use crate::middleware::AuthMiddleware;
use crate::models::user::{self, ActiveModel as UserActiveModel, Entity as User};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub theme_preference: Option<String>,
    pub is_admin: bool,
    pub created_at: String,
    pub last_login: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(max = 50))]
    pub display_name: Option<String>,
    #[validate(length(max = 1000))]
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub theme_preference: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    #[validate(length(min = 6))]
    pub current_password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
}

impl From<user::Model> for UserResponse {
    fn from(user: user::Model) -> Self {
        UserResponse {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            theme_preference: user.theme_preference,
            is_admin: user.is_admin,
            created_at: user.created_at.to_rfc3339(),
            last_login: user.last_login.map(|dt| dt.to_rfc3339()),
        }
    }
}

async fn get_current_user(
    auth_payload: web::ReqData<AuthPayload>,
    pool: web::Data<DatabaseConnection>,
) -> Result<impl Responder, AppError> {
    let user_id = auth_payload.user_id;
    let user = auth::find_user_by_id(pool.get_ref(), user_id)
        .await?
        .ok_or_else(|| AppError::not_found_error("User not found"))?;

    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

async fn update_user(
    auth_payload: web::ReqData<AuthPayload>,
    pool: web::Data<DatabaseConnection>,
    web::Json(req): web::Json<UpdateUserRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    let user_id = auth_payload.user_id;
    let user = auth::find_user_by_id(pool.get_ref(), user_id)
        .await?
        .ok_or_else(|| AppError::not_found_error("User not found"))?;

    // Create an active model from the existing user
    let mut user_model: UserActiveModel = user.into();

    // Update the fields that were provided
    if let Some(display_name) = req.display_name {
        user_model.display_name = Set(Some(display_name));
    }
    if let Some(bio) = req.bio {
        user_model.bio = Set(Some(bio));
    }
    if let Some(avatar_url) = req.avatar_url {
        user_model.avatar_url = Set(Some(avatar_url));
    }
    if let Some(theme_preference) = req.theme_preference {
        user_model.theme_preference = Set(Some(theme_preference));
    }

    // Update the updated_at timestamp
    user_model.updated_at = Set(Utc::now());

    // Save the updated user
    let updated_user = user_model.update(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(UserResponse::from(updated_user)))
}

async fn change_password(
    auth_payload: web::ReqData<AuthPayload>,
    pool: web::Data<DatabaseConnection>,
    web::Json(req): web::Json<ChangePasswordRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    let user_id = auth_payload.user_id;
    let user = auth::find_user_by_id(pool.get_ref(), user_id)
        .await?
        .ok_or_else(|| AppError::not_found_error("User not found"))?;

    // Verify the current password
    let is_valid = password::verify_password(&req.current_password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::authentication_error("Current password is incorrect"));
    }

    // Hash the new password
    let password_hash = password::hash_password(&req.new_password)?;

    // Update the password
    let mut user_model: UserActiveModel = user.into();
    user_model.password_hash = Set(password_hash);
    user_model.updated_at = Set(Utc::now());
    user_model.update(pool.get_ref()).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Password changed successfully",
    })))
}

async fn get_user_by_id(
    pool: web::Data<DatabaseConnection>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let user_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::ValidationError("Invalid user ID format".to_string()))?;

    let user = auth::find_user_by_id(pool.get_ref(), user_id)
        .await?
        .ok_or_else(|| AppError::not_found_error("User not found"))?;

    // Don't expose the email address to other users
    let mut user_response = UserResponse::from(user);
    user_response.email = "".to_string();

    Ok(HttpResponse::Ok().json(user_response))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/me", web::get().to(get_current_user))
            .route("/me", web::put().to(update_user))
            .route("/change-password", web::post().to(change_password))
            .route("/{id}", web::get().to(get_user_by_id))
            .wrap(AuthMiddleware),
    );
} 