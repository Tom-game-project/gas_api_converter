use gas_api_json::{
    convert_wit_type_string, parameters_string, read_service_definition, wit_gen_func_def, Js2WitConvertErr, JsTypeString
};
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

#[test]
fn test_wit_gen_func_def()
{
    let file_path = Path::new("api-def/base.json");
    let result = read_service_definition(file_path);

    if let Ok(api_service) = result 
    {
        for i in api_service.classes {
            println!("class name \"{}\"", i.name);
            for j in i.methods {
                println!("{}", wit_gen_func_def(j));
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
        Err(e) => {
            if let Js2WitConvertErr::NotPrimitiveType(d) = e{

                println!("Gas 独自のクラス: {}", d.0);
            }
            else
            {
                println!("Syntax Error Occured");
            }
        }
    }
    //if let Ok(b) = convert_wit_type_string(
    //    JsTypeString(a.to_string())
    //)
    //{
    //    println!("wit: {}", b.0);
    //}
    //else 
    //{
    //    println!("Error Occured");
    //}
}
