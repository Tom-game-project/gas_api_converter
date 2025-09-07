use gas_api_json::{
    find_type_define_location, is_in_same_service, is_in_somewhere_service, is_self_type, read_all_service_definition, read_service_definition, JsTypeString
};

use std::path::Path;

#[test]
fn test_wit_resolver_is_in_same_service()
{
    let file_path = Path::new("api-def/drive.json");
    let result = read_service_definition(file_path).unwrap();

    let js_type_string = JsTypeString("Blob".to_string());
    println!("{:?}", js_type_string);
    assert_eq!(is_in_same_service(&result, &js_type_string), false);

    let js_type_string = JsTypeString("File".to_string());
    println!("{:?}", js_type_string);
    assert_eq!(is_in_same_service(&result, &js_type_string), true);
}

#[test]
fn test_wit_resolver_is_in_somewhere_service()
{
    let path = "./api-def"; // 対象のディレクトリ

    let service_list = read_all_service_definition(path).unwrap();

    let result = service_list.iter().find(|a|a.service_name == "drive").unwrap();
    let js_type_string = JsTypeString("Blob".to_string());
    println!("{:?}", js_type_string);
    assert_eq!(is_in_same_service(&result, &js_type_string), false);
    assert_eq!(is_in_somewhere_service(&service_list, &js_type_string), true);

    let js_type_string = JsTypeString("File".to_string());
    println!("{:?}", js_type_string);
    assert_eq!(is_in_same_service(&result, &js_type_string), true);
}

#[test]
fn test_find_type_define_location()
{
    let path = "./api-def"; // 対象のディレクトリ

    let service_list = read_all_service_definition(path).unwrap();

    let target = "File";
    if let Some (a) = find_type_define_location(
        &service_list,
        &JsTypeString(target.to_string())
    ) {
        println!("{:?}", a);
    }
    else {
        println!("{} not such type found" , target);
    }
}
