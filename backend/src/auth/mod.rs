pub mod jwt;
pub mod password;

use crate::models::user::{self, Entity as User};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthPayload {
    pub user_id: Uuid,
    pub username: String,
    pub is_admin: bool,
}

// Find user by username
pub async fn find_user_by_username(
    db: &DatabaseConnection,
    username: &str,
) -> Result<Option<user::Model>, anyhow::Error> {
    let user = User::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?;
    Ok(user)
}

// Find user by ID
pub async fn find_user_by_id(
    db: &DatabaseConnection,
    user_id: Uuid,
) -> Result<Option<user::Model>, anyhow::Error> {
    let user = User::find_by_id(user_id).one(db).await?;
    Ok(user)
}

// Find user by email
pub async fn find_user_by_email(
    db: &DatabaseConnection,
    email: &str,
) -> Result<Option<user::Model>, anyhow::Error> {
    let user = User::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await?;
    Ok(user)
} 