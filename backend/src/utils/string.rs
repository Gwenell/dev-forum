use slug::slugify;
use uuid::Uuid;

/// Creates a slug from a string and appends a UUID suffix
/// 
/// This ensures the slug is both human-readable and unique
pub fn create_unique_slug(input: &str) -> String {
    let base_slug = slugify(input);
    let uuid_string = Uuid::new_v4().to_string();
    let uuid_suffix = uuid_string.split_at(8).0;
    
    format!("{}-{}", base_slug, uuid_suffix)
}

// Extract file extension from a filename
pub fn get_file_extension(filename: &str) -> Option<String> {
    let parts: Vec<&str> = filename.split('.').collect();
    if parts.len() > 1 {
        Some(parts.last().unwrap().to_lowercase())
    } else {
        None
    }
}

// Determine if a file extension is allowed
pub fn is_allowed_extension(extension: &str, allowed_extensions: &[&str]) -> bool {
    allowed_extensions.contains(&extension)
}

// Normalize a filename to make it safe for storage
pub fn normalize_filename(filename: &str) -> String {
    // Replace spaces and special characters
    let cleaned = filename
        .chars()
        .map(|c| match c {
            ' ' | '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            _ => c,
        })
        .collect::<String>();
    
    // Add a UUID to ensure uniqueness
    let uuid_string = Uuid::new_v4().to_string();
    let uuid = uuid_string.split_at(8).0;
    
    if let Some(extension) = get_file_extension(&cleaned) {
        let name_without_extension = cleaned.trim_end_matches(&format!(".{}", extension));
        format!("{}_{}.{}", name_without_extension, uuid, extension)
    } else {
        format!("{}_{}", cleaned, uuid)
    }
} 