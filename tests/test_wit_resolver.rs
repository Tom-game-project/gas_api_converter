use gas_api_json::{find_type_define_location, is_in_same_service, is_in_somewhere_service, is_self_type, read_all_service_definition, read_service_definition, wit_gen_func_def, Js2WitConvertErr, JsTypeString, TypeDefineLocation, ApiService
};

use std::{collections::HashSet, path::Path};
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

fn loop_find_type_define_location<'a, T>(self_service_set: &[ApiService], deps_uses: T) -> Vec<TypeDefineLocation>
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

#[test]
fn test_wit_gen_func_def01()
{
    let path = "./api-def"; // 対象のディレクトリ
    let service_list = read_all_service_definition(path).unwrap();
    let result = service_list.iter().find(|a|a.service_name == "drive");

    if let Some(api_service) = result 
    {
        for i in &api_service.classes {
            println!("class name \"{}\"", i.name);
            let mut deps_uses: HashSet<JsTypeString> = HashSet::new();
            for j in &i.methods {
                let a = wit_gen_func_def(j);

                match a{
                    Ok(b) => {
                        println!("Primitive クラス: {}", b.0.green());
                    }
                    Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
                        println!("Primitiveでない: {}", wit_type_string.0.purple());
                        
                        deps_uses.extend(unknown_fields);
                        //println!("次の型の定義の確認が必要です {:?}", unknown_fields);
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
            let type_location = loop_find_type_define_location(&service_list, &deps_uses);
            // 依存する型の列挙
            for i in type_location {
                println!("{:?}", i.cyan())
            }

            //println!("you need to resolve these types {:?} in {} class", deps_uses, i.name);
            println!("===");
        }
    }
    else
    {
        println!("Some Error occured!");
    }
}
