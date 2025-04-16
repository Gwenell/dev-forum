use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
    pub frontend_url: String,
    pub max_upload_size: usize,
}

impl Config {
    pub fn from_env() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse()
            .expect("PORT must be a number");
        let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
        let max_upload_size = env::var("MAX_UPLOAD_SIZE")
            .unwrap_or_else(|_| "20971520".to_string()) // Default: 20MB
            .parse()
            .expect("MAX_UPLOAD_SIZE must be a number");

        Config {
            database_url,
            jwt_secret,
            port,
            frontend_url,
            max_upload_size,
        }
    }
} 