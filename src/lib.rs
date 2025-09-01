use std::fs;
use std::path::Path;

mod wit;
mod json_struct;
use json_struct::*;
pub use wit::parameters_string;

pub fn read_service_definition(file_path: &Path) -> Result<ApiService, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let api_service: ApiService = serde_json::from_str(&content)?;
    Ok(api_service)
}
