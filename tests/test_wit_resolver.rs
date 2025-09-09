use gas_api_json::{find_type_define_location, is_in_same_service, is_in_somewhere_service, is_self_type, read_all_service_definition, read_service_definition, wit_gen_func_def, wit_gen_service_use, ApiService, Js2WitConvertErr, JsTypeString, TypeDefineLocation, WitTypeString,wit_gen_interface_use, wit_gen_interface_name
};

use std::{collections::HashSet, hash::Hash, path::Path};
use owo_colors::OwoColorize;

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

#[test]
fn test_wit_gen_func_def00()
{
    //let file_path = Path::new("api-def/base.json");
    let file_path = Path::new("api-def/drive.json");
    let result = read_service_definition(file_path);

    if let Ok(api_service) = result 
    {
        for i in api_service.classes {
            println!("class name \"{}\"", i.name);
            for j in i.methods {
                let a = wit_gen_func_def(&j);

                match a{
                    Ok(b) => {
                        println!("Primitive クラス: {}", b.0.green());
                    }
                    Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
                        println!("Primitiveでない: {}", wit_type_string.0.purple());

                        println!("次の型の定義の確認が必要です {:?}", unknown_fields);
                    }
                    _ => {
                        println!("{}", "Something Wrong!".red());
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

fn required_find_type_define_location<'a, T>(
    self_service_set: &[ApiService],  // すべてのサービスを格納したリストなど
    deps_uses: T                      // unknown_fieldsから返されたJsTypeStringのリストなど
) -> Vec<TypeDefineLocation>
where 
    T: IntoIterator<Item = &'a JsTypeString>
{
    let mut rlist = vec![];
    for i in deps_uses{
        if let Some(a) =
            find_type_define_location(self_service_set, i)
        {
            rlist.push(a);
        }
    }
    rlist
}

fn eq_type_define_location_and_service(
    type_define_location:&TypeDefineLocation, // 型が定義されている部分の情報
    service: &ApiService                      // サービス(の名前)
) -> bool
{
    type_define_location.service.0 == service.service_name
}

#[test]
fn test_wit_gen_func_def01()
{
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list.iter().find(|a|a.service_name == "drive");

    if let Some(api_service) = result 
    {
        //let mut deps_uses_service = HashSet::new();
        for i in &api_service.classes {
            println!("class name \"{}\"", i.name);
            let mut deps_uses: HashSet<JsTypeString> = HashSet::new();
            for j in &i.methods {
                match wit_gen_func_def(j){
                    Ok(b) => {
                        println!("Primitive クラス: {}", b.0.green());
                    }
                    Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
                        println!("Primitiveでない: {}", wit_type_string.0.purple());
                        
                        deps_uses.extend(unknown_fields);
                        //println!("次の型の定義の確認が必要です {:?}", unknown_fields);
                    }
                    _ => {
                        println!("{}", "Something Wrong!".red());
                    }
                }
            }
            let type_location = 
                required_find_type_define_location(&service_list, &deps_uses);
            // 依存する型の列挙

            // JsTypeString(api_service.service_name) == 

            let (in_of_service, out_of_service):(Vec<_>, Vec<_>) =
                type_location
                    .into_iter()
                    .partition(
                        |inner| 
                        eq_type_define_location_and_service(inner, &api_service) 
                        // 型の定義がサービスファイル内にあるかどうか？
                    );

            println!("=== in of service ===");
            for i in &in_of_service {
                if let Ok(a) = 
                    wit_gen_interface_use(&i.class)
                {
                    println!("{}",
                        a.0.cyan()
                    );
                }
                else 
                {
                    println!("{}", "Something Wrong".red());
                }
            }
            println!("=== out of service ===");
            for i in  &out_of_service {
                if let Ok(a) = 
                    wit_gen_service_use(
                        "gas", 
                        &i.service,
                        &i.class,
                        Some(
                            &WitTypeString("alpha-0.1.0".to_string())
                        )
                )
                {
                    println!("{}", a.0.blue())
                    
                }
                else
                {
                    println!("{}", "Something Wrong!".red());
                }
                if let Ok(a) = 
                    wit_gen_interface_use(&i.class)
                {
                    println!("{}",
                        a.0.cyan()
                    );
                }
                else 
                {
                    println!("{}", "Something Wrong".red());
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
fn eq_js_type_string_test00()
{
    assert!(
        JsTypeString("hello world".to_string()) == JsTypeString("hello world".to_string())
    );
    assert!(
        JsTypeString("world".to_string()) != JsTypeString("hello".to_string())
    );
}

#[test]
fn test_wit_gen_interface_name()
{
    if let Ok(a) = wit_gen_interface_name(
        &JsTypeString("Class FolderIterator".to_string())
    )
    {
        println!("{}", a.0);
    }
    else 
    {
        println!("Error");
    }
}

