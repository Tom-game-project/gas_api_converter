use gas_api_json::{generate_wit_definition, generate_wit_definition_string, generate_wit_definition_with_filter, read_all_service_definition, JsTypeString, WitDefFile};
use owo_colors::OwoColorize;


/// witdefineオブジェクトの生成とそれに基づいたコード生成のテスト
#[test]
fn test_wit_generator00()
{
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list.iter().find(|a|a.service_name == "base").unwrap();

    //let wdf = WitDefFile::new(result, &service_list);

    //let wit_def = generate_wit_definition(&wdf);

    //println!("{}", wit_def);
}


#[test]
fn test_wit_generator01()
{
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list.iter().find(|a|a.service_name == "drive").unwrap();

    let r = generate_wit_definition(
        &result,
        &service_list
    );

    if let Ok(wit_def_file) = r {
        let wit_def = generate_wit_definition_string(&wit_def_file);
        println!("{}", wit_def);
    }
    else if let Err(e) = r
    {
        println!("{:?}", e.red());
    }
}

#[test]
fn test_wit_generator02()
{
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list.iter().find(|a|a.service_name == "drive").unwrap();

    let r = generate_wit_definition_with_filter(
        &result,
        &service_list,
        &vec![
            JsTypeString("FolderIterator".to_string()),
            JsTypeString("FileIterator".to_string()),
            JsTypeString("File".to_string()),
            JsTypeString("Folder".to_string()),
            JsTypeString("Blob".to_string()),
            JsTypeString("DriveApp".to_string()),
        ]
    );

    if let Ok(wit_def_file) = r {
        let wit_def = generate_wit_definition_string(&wit_def_file);
        println!("{}", wit_def);
    }
    else if let Err(e) = r
    {
        println!("{:?}", e.red());
    }
}
