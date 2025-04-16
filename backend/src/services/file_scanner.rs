#[cfg(feature = "clamav")]
use clamav_rs::engine::{Engine, ScanResult, ScanSettings};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[cfg(feature = "clamav")]
pub struct FileScanner {
    engine: Engine,
}

#[cfg(feature = "clamav")]
impl FileScanner {
    pub fn new() -> Result<Self, anyhow::Error> {
        // Initialize ClamAV
        let engine = Engine::new()?;
        
        // Load the latest virus definitions
        engine.load_databases()?;
        
        Ok(FileScanner { engine })
    }

    pub fn scan_file<P: AsRef<Path>>(&self, path: P) -> Result<bool, anyhow::Error> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        let settings = ScanSettings::default();
        let result = self.engine.scan_bytes(&buffer, &settings)?;
        
        match result {
            ScanResult::Clean => Ok(true),
            ScanResult::Virus(name) => {
                log::warn!("Found virus: {}", name);
                Ok(false)
            }
            ScanResult::Error(err) => {
                Err(anyhow::anyhow!("Error scanning file: {}", err))
            }
        }
    }
}

#[cfg(feature = "clamav")]
impl Default for FileScanner {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ClamAV engine")
    }
}

/// Scans a file for malware
/// 
/// Returns true if the file is safe, false if it's not
pub async fn scan_file(file_path: &Path) -> Result<bool, String> {
    #[cfg(feature = "clamav")]
    {
        // Initialize ClamAV
        match Engine::new() {
            Ok(mut engine) => {
                // Load the virus database
                match engine.load_databases() {
                    Ok(_) => {
                        // Create scan settings
                        let settings = ScanSettings::default();
                        
                        // Scan the file
                        match engine.scan_file(file_path, &settings) {
                            Ok(ScanResult::Clean) => Ok(true),
                            Ok(ScanResult::Virus(name)) => {
                                log::warn!("Virus detected: {} in file: {:?}", name, file_path);
                                Ok(false)
                            }
                            Err(e) => Err(format!("Error scanning file: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Could not load virus database: {}", e)),
                }
            }
            Err(e) => Err(format!("Could not initialize ClamAV: {}", e)),
        }
    }
    
    #[cfg(not(feature = "clamav"))]
    {
        log::warn!("ClamAV is not enabled, skipping virus scan for: {:?}", file_path);
        // Return true (safe) when ClamAV is not available
        Ok(true)
    }
} 