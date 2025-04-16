use crate::auth::AuthPayload;
use crate::db::DbPool;
use crate::error::AppError;
use crate::middleware::{AdminMiddleware, AuthMiddleware};
use crate::models::{
    category::{self, Entity as Category},
    subcategory::{self, Entity as Subcategory},
};
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait,
    Set,
};
use serde::{Deserialize, Serialize};
use slug::slugify;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub display_order: i32,
    pub subcategories: Vec<SubcategoryResponse>,
}

#[derive(Debug, Serialize)]
pub struct SubcategoryResponse {
    pub id: String,
    pub category_id: String,
    pub name: String,
    pub slug: String,
    pub description: String,
    pub icon: Option<String>,
    pub display_order: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 10, max = 1000))]
    pub description: String,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    #[validate(length(min = 10, max = 1000))]
    pub description: Option<String>,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateSubcategoryRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(length(min = 10, max = 1000))]
    pub description: String,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateSubcategoryRequest {
    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,
    #[validate(length(min = 10, max = 1000))]
    pub description: Option<String>,
    pub icon: Option<String>,
    pub display_order: Option<i32>,
}

impl From<subcategory::Model> for SubcategoryResponse {
    fn from(subcategory: subcategory::Model) -> Self {
        SubcategoryResponse {
            id: subcategory.id.to_string(),
            category_id: subcategory.category_id.to_string(),
            name: subcategory.name,
            slug: subcategory.slug,
            description: subcategory.description.unwrap_or_default(),
            icon: subcategory.icon,
            display_order: subcategory.display_order,
        }
    }
}

// Get all categories with subcategories
async fn get_categories(pool: web::Data<DbPool>) -> Result<impl Responder, AppError> {
    let categories = Category::find()
        .order_by_asc(category::Column::DisplayOrder)
        .all(pool.as_ref())
        .await?;

    let mut response = Vec::new();
    for category in categories {
        // Get subcategories for this category
        let subcategories = Subcategory::find()
            .filter(subcategory::Column::CategoryId.eq(category.id))
            .order_by_asc(subcategory::Column::DisplayOrder)
            .all(pool.as_ref())
            .await?;

        let subcategory_responses = subcategories
            .into_iter()
            .map(SubcategoryResponse::from)
            .collect();

        response.push(CategoryResponse {
            id: category.id.to_string(),
            name: category.name,
            slug: category.slug,
            description: Some(category.description.unwrap_or_default()),
            icon: category.icon,
            display_order: category.display_order,
            subcategories: subcategory_responses,
        });
    }

    Ok(HttpResponse::Ok().json(response))
}

// Get a category by ID
async fn get_category(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let category_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::ValidationError("Invalid category ID format".to_string()))?;

    let category = Category::find_by_id(category_id)
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Category not found"))?;

    // Get subcategories for this category
    let subcategories = Subcategory::find()
        .filter(subcategory::Column::CategoryId.eq(category.id))
        .order_by_asc(subcategory::Column::DisplayOrder)
        .all(pool.as_ref())
        .await?;

    let subcategory_responses = subcategories
        .into_iter()
        .map(SubcategoryResponse::from)
        .collect();

    let response = CategoryResponse {
        id: category.id.to_string(),
        name: category.name,
        slug: category.slug,
        description: Some(category.description.unwrap_or_default()),
        icon: category.icon,
        display_order: category.display_order,
        subcategories: subcategory_responses,
    };

    Ok(HttpResponse::Ok().json(response))
}

// Get a category by slug
async fn get_category_by_slug(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let slug = path.into_inner();

    let category = Category::find()
        .filter(category::Column::Slug.eq(slug))
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Category not found"))?;

    // Get subcategories for this category
    let subcategories = Subcategory::find()
        .filter(subcategory::Column::CategoryId.eq(category.id))
        .order_by_asc(subcategory::Column::DisplayOrder)
        .all(pool.as_ref())
        .await?;

    let subcategory_responses = subcategories
        .into_iter()
        .map(SubcategoryResponse::from)
        .collect();

    let response = CategoryResponse {
        id: category.id.to_string(),
        name: category.name,
        slug: category.slug,
        description: Some(category.description.unwrap_or_default()),
        icon: category.icon,
        display_order: category.display_order,
        subcategories: subcategory_responses,
    };

    Ok(HttpResponse::Ok().json(response))
}

// Create a new category (admin only)
async fn create_category(
    pool: web::Data<DbPool>,
    web::Json(req): web::Json<CreateCategoryRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    // Generate a slug from the name
    let slug = slugify(&req.name);

    // Check if a category with this slug already exists
    let existing_category = Category::find()
        .filter(category::Column::Slug.eq(&slug))
        .one(pool.as_ref())
        .await?;

    if existing_category.is_some() {
        return Err(AppError::ValidationError("A category with this name already exists".to_string()));
    }

    // Create the category
    let category = category::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(req.name),
        slug: Set(slug),
        description: Set(Some(req.description)),
        icon: Set(req.icon),
        display_order: Set(req.display_order.unwrap_or(0)),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let category = category.insert(pool.as_ref()).await?;

    let response = CategoryResponse {
        id: category.id.to_string(),
        name: category.name,
        slug: category.slug,
        description: Some(category.description.unwrap_or_default()),
        icon: category.icon,
        display_order: category.display_order,
        subcategories: Vec::new(),
    };

    Ok(HttpResponse::Created().json(response))
}

// Update a category (admin only)
async fn update_category(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    web::Json(req): web::Json<UpdateCategoryRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    let category_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::ValidationError("Invalid category ID format".to_string()))?;

    // Find the category
    let category = Category::find_by_id(category_id)
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Category not found"))?;

    // Create an active model from the existing category
    let mut category_model: category::ActiveModel = category.into();

    // Check if name is being updated
    if let Some(name) = req.name {
        let slug = slugify(&name);

        // Check if another category with this slug already exists
        let existing_category = Category::find()
            .filter(category::Column::Slug.eq(&slug))
            .filter(category::Column::Id.ne(category_id))
            .one(pool.as_ref())
            .await?;

        if existing_category.is_some() {
            return Err(AppError::ValidationError("A category with this name already exists".to_string()));
        }

        category_model.name = Set(name);
        category_model.slug = Set(slug);
    }

    // Update other fields if provided
    if let Some(description) = req.description {
        category_model.description = Set(Some(description));
    }
    if let Some(icon) = req.icon {
        category_model.icon = Set(Some(icon));
    }
    if let Some(display_order) = req.display_order {
        category_model.display_order = Set(display_order);
    }

    // Update the updated_at timestamp
    category_model.updated_at = Set(Utc::now());

    // Save the updated category
    let updated_category = category_model.update(pool.as_ref()).await?;

    // Get subcategories for this category
    let subcategories = Subcategory::find()
        .filter(subcategory::Column::CategoryId.eq(updated_category.id))
        .order_by_asc(subcategory::Column::DisplayOrder)
        .all(pool.as_ref())
        .await?;

    let subcategory_responses = subcategories
        .into_iter()
        .map(SubcategoryResponse::from)
        .collect();

    let response = CategoryResponse {
        id: updated_category.id.to_string(),
        name: updated_category.name,
        slug: updated_category.slug,
        description: Some(updated_category.description.unwrap_or_default()),
        icon: updated_category.icon,
        display_order: updated_category.display_order,
        subcategories: subcategory_responses,
    };

    Ok(HttpResponse::Ok().json(response))
}

// Delete a category (admin only)
async fn delete_category(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let category_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::ValidationError("Invalid category ID format".to_string()))?;

    // Find the category
    let category = Category::find_by_id(category_id)
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Category not found"))?;

    // Delete the category (this will also delete all subcategories due to the cascade constraint)
    let category_model: category::ActiveModel = category.into();
    category_model.delete(pool.as_ref()).await?;

    Ok(HttpResponse::NoContent().finish())
}

// Create a new subcategory (admin only)
async fn create_subcategory(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    web::Json(req): web::Json<CreateSubcategoryRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    let category_id = Uuid::parse_str(&path.into_inner())
        .map_err(|_| AppError::ValidationError("Invalid category ID format".to_string()))?;

    // Find the category
    let category = Category::find_by_id(category_id)
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Category not found"))?;

    // Generate a slug from the name
    let slug = slugify(&req.name);

    // Check if a subcategory with this slug already exists in this category
    let existing_subcategory = Subcategory::find()
        .filter(subcategory::Column::CategoryId.eq(category_id))
        .filter(subcategory::Column::Slug.eq(&slug))
        .one(pool.as_ref())
        .await?;

    if existing_subcategory.is_some() {
        return Err(AppError::ValidationError("A subcategory with this name already exists in this category".to_string()));
    }

    // Create the subcategory
    let subcategory = subcategory::ActiveModel {
        id: Set(Uuid::new_v4()),
        category_id: Set(category_id),
        name: Set(req.name),
        slug: Set(slug),
        description: Set(Some(req.description)),
        icon: Set(req.icon),
        display_order: Set(req.display_order.unwrap_or(0)),
        created_at: Set(Utc::now()),
        updated_at: Set(Utc::now()),
    };

    let subcategory = subcategory.insert(pool.as_ref()).await?;
    let response = SubcategoryResponse::from(subcategory);

    Ok(HttpResponse::Created().json(response))
}

// Update a subcategory (admin only)
async fn update_subcategory(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
    web::Json(req): web::Json<UpdateSubcategoryRequest>,
) -> Result<impl Responder, AppError> {
    // Validate the request
    req.validate()?;

    let (category_id_str, subcategory_id_str) = path.into_inner();
    let category_id = Uuid::parse_str(&category_id_str)
        .map_err(|_| AppError::ValidationError("Invalid category ID format".to_string()))?;
    let subcategory_id = Uuid::parse_str(&subcategory_id_str)
        .map_err(|_| AppError::ValidationError("Invalid subcategory ID format".to_string()))?;

    // Find the subcategory
    let subcategory = Subcategory::find_by_id(subcategory_id)
        .filter(subcategory::Column::CategoryId.eq(category_id))
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Subcategory not found"))?;

    // Create an active model from the existing subcategory
    let mut subcategory_model: subcategory::ActiveModel = subcategory.into();

    // Check if name is being updated
    if let Some(name) = req.name {
        let slug = slugify(&name);

        // Check if another subcategory with this slug already exists in this category
        let existing_subcategory = Subcategory::find()
            .filter(subcategory::Column::CategoryId.eq(category_id))
            .filter(subcategory::Column::Slug.eq(&slug))
            .filter(subcategory::Column::Id.ne(subcategory_id))
            .one(pool.as_ref())
            .await?;

        if existing_subcategory.is_some() {
            return Err(AppError::ValidationError("A subcategory with this name already exists in this category".to_string()));
        }

        subcategory_model.name = Set(name);
        subcategory_model.slug = Set(slug);
    }

    // Update other fields if provided
    if let Some(description) = req.description {
        subcategory_model.description = Set(Some(description));
    }
    if let Some(icon) = req.icon {
        subcategory_model.icon = Set(Some(icon));
    }
    if let Some(display_order) = req.display_order {
        subcategory_model.display_order = Set(display_order);
    }

    // Update the updated_at timestamp
    subcategory_model.updated_at = Set(Utc::now());

    // Save the updated subcategory
    let updated_subcategory = subcategory_model.update(pool.as_ref()).await?;
    let response = SubcategoryResponse::from(updated_subcategory);

    Ok(HttpResponse::Ok().json(response))
}

// Delete a subcategory (admin only)
async fn delete_subcategory(
    pool: web::Data<DbPool>,
    path: web::Path<(String, String)>,
) -> Result<impl Responder, AppError> {
    let (category_id_str, subcategory_id_str) = path.into_inner();
    let category_id = Uuid::parse_str(&category_id_str)
        .map_err(|_| AppError::ValidationError("Invalid category ID format".to_string()))?;
    let subcategory_id = Uuid::parse_str(&subcategory_id_str)
        .map_err(|_| AppError::ValidationError("Invalid subcategory ID format".to_string()))?;

    // Find the subcategory
    let subcategory = Subcategory::find_by_id(subcategory_id)
        .filter(subcategory::Column::CategoryId.eq(category_id))
        .one(pool.as_ref())
        .await?
        .ok_or_else(|| AppError::not_found_error("Subcategory not found"))?;

    // Delete the subcategory
    let subcategory_model: subcategory::ActiveModel = subcategory.into();
    subcategory_model.delete(pool.as_ref()).await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            // Public routes
            .route("", web::get().to(get_categories))
            .route("/{id}", web::get().to(get_category))
            .route("/slug/{slug}", web::get().to(get_category_by_slug))
            // Admin routes
            .service(
                web::scope("/admin")
                    .route("", web::post().to(create_category))
                    .route("/{id}", web::put().to(update_category))
                    .route("/{id}", web::delete().to(delete_category))
                    .route("/{id}/subcategories", web::post().to(create_subcategory))
                    .route(
                        "/{category_id}/subcategories/{subcategory_id}",
                        web::put().to(update_subcategory),
                    )
                    .route(
                        "/{category_id}/subcategories/{subcategory_id}",
                        web::delete().to(delete_subcategory),
                    )
                    .wrap(AdminMiddleware)
                    .wrap(AuthMiddleware),
            ),
    );
} 