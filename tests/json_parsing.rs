use gas_api_json::{
    convert_wit_type_string2, read_all_service_definition, read_service_definition, wit_gen_func_def, wit_parameters_string, Js2WitConvertErr, JsTypeString, Type
};

use std::path::Path;
use owo_colors::OwoColorize;

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
fn test_convert_wit_type_string()
{
    let a = "Blob[][]";

    match convert_wit_type_string2(
             &Type {
                 name: a.to_string(),
                 url: None
             }
    ){
        Ok(b) => {
            println!("Primitive クラス: {}", b.0);
        }
        Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
            println!("Gas 独自のクラス: {}", wit_type_string.0);
        }
        _ => {
            println!("{}", "Something Wrong!".red());
        }
    }
}

#[test]
fn interpret_all_service()
{
    let path = "./api-def"; // 対象のディレクトリ

    let a = read_all_service_definition(path).unwrap();

    for i in a {
        println!("service_name {}", i.service_name);
    }
}
