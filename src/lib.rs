use std::fs;
use std::path::Path;

mod wit;
mod json_struct;
mod wit_resolver;
use json_struct::*;
pub use wit::{
    wit_parameters_string,
    wit_gen_func_def,
    convert_wit_type_string,
    JsTypeString,
    WitTypeString,
    Js2WitConvertErr,
};

pub use wit_resolver::{
    is_self_type,
    is_in_same_service,
    is_in_somewhere_service,
};

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
