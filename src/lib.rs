use std::fs;
use std::path::Path;

mod wit;
mod json_struct;
mod wit_resolver;

pub use json_struct::*;
pub use wit::*;
pub use wit_resolver::*;

pub fn read_service_definition(file_path: &Path) -> Result<ApiService, Box<dyn std::error::Error>> 
{
    let content = fs::read_to_string(file_path)?;
    let api_service: ApiService = serde_json::from_str(&content)?;
    Ok(api_service)
}

pub fn read_all_service_definition(dir_path: &str) -> Result<Vec<ApiService>, Box<dyn std::error::Error>> 
{
    let mut rlist = vec![];
    for entry in fs::read_dir(dir_path)? {
        let entry = entry.unwrap();
        let file_path = entry.path();

        let content = fs::read_to_string(file_path)?;
        let api_service: ApiService = serde_json::from_str(&content)?;
        rlist.push(api_service);
    }
    Ok(rlist)
}
