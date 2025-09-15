use gas_api_json::read_all_service_definition;

#[test]
fn test00() {
    let path = "./api-def"; // 対象のディレクトリ

    let service_list = read_all_service_definition(path).unwrap();

    let result = service_list
        .iter()
        .find(|a| a.service_name == "drive")
        .unwrap();

    let a = result
        .classes
        .iter()
        .find(|i| i.name == "Enum Access")
        .unwrap();
    println!("Enum Access");
    for i in &a.enum_members {
        println!("    {}", i.name)
    }
}
