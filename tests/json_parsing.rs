use gas_api_json::{read_service_definition, parameters_string};
use std::path::Path;

#[test]
fn test_parse_base_json00() {
    let file_path = Path::new("api-def/base.json");
    let result = read_service_definition(file_path);

    // Check if parsing was successful
    assert!(result.is_ok(), "Failed to parse base.json: {:?}", result.err());

    let api_service = result.unwrap();

    // Verify some top-level data
    assert_eq!(api_service.service_name, "base");
    assert!(!api_service.classes.is_empty(), "Classes vector should not be empty");

    // Verify some nested data to ensure deep parsing works
    let first_class = &api_service.classes[0];
    assert_eq!(first_class.name, "Class Blob");
    assert!(!first_class.methods.is_empty(), "Methods vector should not be empty");

    let first_method = &first_class.methods[0];
    assert_eq!(first_method.name, "copyBlob()");
    assert_eq!(first_method.return_type.name, "Blob");
    assert!(first_method.parameters.is_empty(), "copyBlob should have no parameters");
}

#[test]
fn test_parse_base_json01()
{
    let file_path = Path::new("api-def/base.json");
    let result = read_service_definition(file_path);

    if let Ok(api_service) = result {
        for i in api_service.classes {
            println!("class name \"{}\"", i.name);
            for j in i.methods {
                println!("    method {}: {}",
                    j.name,
                    format!("func ({}) -> {}", 
                        parameters_string(j.parameters),
                        j.return_type.name));
            }
            println!("===");
        }
    } 
    else
    {
        println!("Some Error occured!");
    }
}
