use gas_api_json::{
    convert_wit_type_string, 
    wit_parameters_string, 
    wit_gen_func_def,
    read_service_definition, 
    Js2WitConvertErr, 
    JsTypeString,
    read_all_service_definition
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
fn test_wit_gen_func_def()
{
    //let file_path = Path::new("api-def/base.json");
    let file_path = Path::new("api-def/drive.json");
    let result = read_service_definition(file_path);

    if let Ok(api_service) = result 
    {
        for i in api_service.classes {
            println!("class name \"{}\"", i.name);
            for j in i.methods {
                let a = wit_gen_func_def(j);

                match a{
                    Ok(b) => {
                        println!("Primitive クラス: {}", b.0.green());
                    }
                    Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
                        println!("Primitiveでない: {}", wit_type_string.0.purple());

                        println!("次の型の定義の確認が必要です {:?}", unknown_fields);
                    }
                    Err(Js2WitConvertErr::SyntaxErr) => {
                        println!("Syntax Error!");
                    }
                    Err(Js2WitConvertErr::ParameterStringErr) => {
                        println!("{}", "could not interpret parameter string".red());
                    }
                    Err(Js2WitConvertErr::ReturnStringErr) => {
                        println!("{}", "could not interpret return string".red());
                    }
                }
            }
            println!("===");
        }
    
    }
    else
    {
        println!("Some Error occured!");
    }
}

#[test]
fn test_convert_wit_type_string()
{
    let a = "Blob[][]";

    match convert_wit_type_string(
        JsTypeString(a.to_string())
    ){
        Ok(b) => {
            println!("Primitive クラス: {}", b.0);
        }
        Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
            println!("Gas 独自のクラス: {}", wit_type_string.0);
        }
        Err(Js2WitConvertErr::SyntaxErr) => {
            println!("Syntax Error!");
        }
        Err(Js2WitConvertErr::ParameterStringErr) => {
            println!("could not interpret parameter string");
        }
        Err(Js2WitConvertErr::ReturnStringErr) => {
            println!("could not interpret return string");
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
