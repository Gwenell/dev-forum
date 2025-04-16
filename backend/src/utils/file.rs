use crate::error::AppError;
use crate::services::file_scanner::FileScanner;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

// Ensure the uploads directory exists
pub fn ensure_upload_dir(dir: &str) -> Result<(), AppError> {
    if !Path::new(dir).exists() {
        fs::create_dir_all(dir).map_err(|e| AppError::internal_server_error(format!(
            "Failed to create upload directory: {}", e
        )))?;
    }
    Ok(())
}

// Save a file to disk
pub fn save_file(data: &[u8], path: &Path) -> Result<(), AppError> {
    let mut file = File::create(path).map_err(|e| {
        AppError::internal_server_error(format!("Failed to create file: {}", e))
    })?;
    
    file.write_all(data).map_err(|e| {
        AppError::internal_server_error(format!("Failed to write to file: {}", e))
    })?;
    
    Ok(())
}

// Scan a file for malware
pub fn scan_file_for_malware<P: AsRef<Path>>(path: P) -> Result<bool, AppError> {
    let scanner = FileScanner::new()
        .map_err(|e| AppError::file_scanning_error(format!("Failed to initialize scanner: {}", e)))?;
    
    scanner.scan_file(path)
        .map_err(|e| AppError::file_scanning_error(format!("Failed to scan file: {}", e)))
}

// Delete a file
pub fn delete_file<P: AsRef<Path>>(path: P) -> Result<(), AppError> {
    fs::remove_file(path).map_err(|e| {
        AppError::internal_server_error(format!("Failed to delete file: {}", e))
    })
}

// Get the MIME type of a file based on extension
pub fn get_mime_type(extension: &str) -> String {
    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg".to_string(),
        "png" => "image/png".to_string(),
        "gif" => "image/gif".to_string(),
        "webp" => "image/webp".to_string(),
        "pdf" => "application/pdf".to_string(),
        "zip" => "application/zip".to_string(),
        "rar" => "application/x-rar-compressed".to_string(),
        "7z" => "application/x-7z-compressed".to_string(),
        "gz" | "tar.gz" => "application/gzip".to_string(),
        "tar" => "application/x-tar".to_string(),
        "exe" => "application/x-msdownload".to_string(),
        "apk" => "application/vnd.android.package-archive".to_string(),
        "dmg" => "application/x-apple-diskimage".to_string(),
        "deb" => "application/vnd.debian.binary-package".to_string(),
        "rpm" => "application/x-rpm".to_string(),
        "txt" => "text/plain".to_string(),
        "html" | "htm" => "text/html".to_string(),
        "css" => "text/css".to_string(),
        "js" => "application/javascript".to_string(),
        "json" => "application/json".to_string(),
        "xml" => "application/xml".to_string(),
        "csv" => "text/csv".to_string(),
        "doc" | "docx" => "application/msword".to_string(),
        "xls" | "xlsx" => "application/vnd.ms-excel".to_string(),
        "ppt" | "pptx" => "application/vnd.ms-powerpoint".to_string(),
        _ => "application/octet-stream".to_string(),
    }
} 