use gas_api_json::{
    JsTypeString, WitDefFile, generate_wit_definition_string, generate_wit_definition_with_filter,
    read_all_service_definition,
};
use owo_colors::OwoColorize;

/// witdefineオブジェクトの生成とそれに基づいたコード生成のテスト
#[test]
fn test_wit_generator00() {
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list
        .iter()
        .find(|a| a.service_name == "base")
        .unwrap();
}

#[test]
fn test_wit_generator01() {
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list
        .iter()
        .find(|a| a.service_name == "drive")
        .unwrap();
}

/// 取り扱う型を制限して生成できるか確かめるテスト
#[test]
fn test_wit_generator02() {
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list
        .iter()
        .find(|a| a.service_name == "drive")
        .unwrap();

    let allowed_interfaces = &vec![
        JsTypeString("FolderIterator".to_string()),
        JsTypeString("FileIterator".to_string()),
        JsTypeString("File".to_string()),
        JsTypeString("Folder".to_string()),
        JsTypeString("Blob".to_string()),
        JsTypeString("DriveApp".to_string()),
    ];
    println!("以下のInterfaceを生成");
    for i in allowed_interfaces {
        println!("{:?}", i.blue());
    }

    let r = generate_wit_definition_with_filter(&result, &service_list, &allowed_interfaces);

    println!("{:#?}", r);
}
