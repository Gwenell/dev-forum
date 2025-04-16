pub mod auth;
pub mod users;
pub mod categories;
// pub mod threads;
// pub mod posts;
// pub mod files;
// pub mod chat;
// pub mod streams;

use actix_web::web;
use crate::db::DbPool;

// Register all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::configure_routes)
            .configure(users::configure_routes)
            .configure(categories::configure_routes)
            // .configure(threads::configure_routes)
            // .configure(posts::configure_routes)
            // .configure(files::configure_routes)
            // .configure(chat::configure_routes)
            // .configure(streams::configure_routes),
    );
} 