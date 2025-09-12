use gas_api_json::{generate_wit_definition, read_all_service_definition, WitDefFile};


/// witdefineオブジェクトの生成とそれに基づいたコード生成のテスト
#[test]
fn test_wit_generator00()
{
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list.iter().find(|a|a.service_name == "spreadsheet").unwrap();



    let wdf = WitDefFile::new(result, &service_list);



    let wit_def = generate_wit_definition(&wdf);

    println!("{}", wit_def);
}
