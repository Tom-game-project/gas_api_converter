use crate::wit::WitTypeString;


struct WitDefFile
{
    package_name: String,
    package_version: Option<String>,
    deps_uses: Vec<String>, // use ...; サービスを超えて必要になるinterfaceのimport
    defined_interfaces: Vec<WitInterface>,
    world_section: WitWorldSection,
}

struct WitInterface
{
    name: String,
    /// (example: blob, file)
    deps_uses: Vec<String>, 
    inner_data: WitDataType,
}

enum WitDataType
{
    /// 自分自身が要素として返却されないクラス
    WitInterfaceConst(WitInterfaceConst),
    /// 自分自身が要素として返却されるクラス
    WitInterfaceResource(WitInterfaceResource),
    /// 列挙型
    WitInterfaceEnum(WitInterfaceEnum)
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値とならないような型
struct WitInterfaceConst
{
    name: String,
    func_defines:Vec<String>,
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値となるような型
struct WitInterfaceResource
{
    name: String,
    func_defines:Vec<String>,
}

struct WitInterfaceEnum
{
    name: String,
    enum_members: Vec<String>,
}

/// Use文のためのセクション
/// witファイル先頭で使う場合(BeyondService)
/// と、ファイルの中のinterfaceを超えて型を利用する場合(BetondInterface)
/// で使い分ける

struct WitWorldSection
{
    imports: Vec<String>,
    exports: Vec<String>,
}


/// witを生成する
fn generate_wit_definition(wit_def_file: &WitDefFile)
{
    let mut rlist = vec![];

    rlist.push(format!("gas:{}{};",
            wit_def_file.package_name,   // サービス(パッケージ)の名前
            wit_def_file.package_version // サービス(サービス)のバージョン
                .clone()
                .map_or(String::new(), |i| format!("@{}", i))
    ));

    // ==== useセクション(サービス全体で必要となるインターフェイスのimport) ====
    for i in &wit_def_file.deps_uses{
        rlist.push(format!("{};", i));
    }

    for i in &wit_def_file.defined_interfaces
    {
        rlist.push(generate_wit_interface_string(i));
    }
    return ;
}

fn generate_wit_uses(deps_uses: &[String]) -> Vec<String>
{
    deps_uses
        .iter()
        .map(|i| format!("use {}.{{{}}}", i, i))
        .collect()
}

fn generate_wit_interface_string(wit_interface: &WitInterface) -> String
{
    // wit_interface.name
    let rstring = format!(
"{} {{
{}
{}
}}",
    wit_interface.name,
    generate_wit_uses(&wit_interface.deps_uses)
    .iter()
    .map(|i| format!("    {};",i))
    .collect::<String>(),
generate_wit_inner_struct(&wit_interface.inner_data), // TODO : wit_interface.inner_data
);
    rstring
}

fn generate_wit_inner_struct(wit_data_type: &WitDataType) -> String
{
    match wit_data_type {
        WitDataType::WitInterfaceConst(inner) => {
            format!("
    interface {} {{
{}
    }}",
    inner.name, 
    inner.func_defines
        .iter()
        .map(|i| format!("        {};", i))
        .collect::<String>())
        }
        WitDataType::WitInterfaceEnum(inner) => {
            format!("
    interface {} {{
        enum {} {{
{}
        }}
    }}",
    inner.name, 
    inner.name, 
    inner.enum_members
        .iter()
        .map(|i| format!("            {};", i))
        .collect::<String>())
        }
        WitDataType::WitInterfaceResource(inner) => {
            format!("
    interface {} {{
        resource {} {{
{}
        }}
    }}",
    inner.name, 
    inner.name, 
    inner.func_defines
        .iter()
        .map(|i| format!("            {};", i))
        .collect::<String>())
        }
    }
}

use crate::{find_type_define_location, get_interface_name_from_js_type, wit_gen_func_def, wit_gen_interface_use, wit_gen_service_use, Class, InterfaceType, Js2WitConvertErr, Method, TypeDefineLocation};
use crate::{ json_struct::{ApiService}, 
    wit::JsTypeString
};

use std::{collections::HashSet, hash::Hash, path::Path};

use owo_colors::OwoColorize;

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

fn get_func_def_string_vec(methods: &[Method]) -> Result<(Vec<String>, HashSet<JsTypeString>), Js2WitConvertErr>
{
    let mut rlist = vec![];
    let mut deps_uses: HashSet<JsTypeString> = HashSet::new();
    for j in methods {
        match wit_gen_func_def(j){
            Ok(b) => {
                //println!("Primitive クラス: {}", b.0.green());
                rlist.push(b.0);
            }
            Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) => {
                //println!("Primitiveでない: {}", wit_type_string.0.purple());
                rlist.push(wit_type_string.0);
                deps_uses.extend(unknown_fields);
                //println!("次の型の定義の確認が必要です {:?}", unknown_fields);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok((rlist, deps_uses))
}


/// 目的の
/// クラスのメソッドや
/// 列挙型のメンバーをwitのデータに変換する
///
/// 関数宣言時に必要になった型定義の要請を返り値の情報に含む
/// WitInterface
fn generate_wit_data_type(
    class: &Class,
    target_service: &ApiService,
    service_list: &[ApiService]
) -> Result<WitInterface, Js2WitConvertErr>
{
    let js_type = &JsTypeString(class.name.clone());
    let inter_face_type = get_interface_name_from_js_type(js_type);
    if let Some((InterfaceType::Class, class_name)) = inter_face_type 
    {
        if let Ok((
            func_def_string_list,  // 関数の定義
            deps_uses    // interfaceが要求する型の定義
        )) = get_func_def_string_vec(&class.methods)
        {
            // TODO: ここでは他のメソッドから返される可能性のある関数か否かで分岐がある
            let type_location = 
                required_find_type_define_location(
                    &service_list,
                    &deps_uses
                );
            // 依存する型の列挙

            let (
                in_of_service, // サービス内に定義された型
                out_of_service // サービス外で定義された型
            ):(Vec<_>, Vec<_>) =
                type_location
                    .into_iter()
                    .partition(
                        |inner| 
                        eq_type_define_location_and_service(inner, &target_service) 
                        // 型の定義がサービスファイル内にあるかどうか？
                        //
                    );

            return Ok(
                WitInterface { 
                    name: class_name.to_string(),
                    deps_uses: todo!(),
                    inner_data:
                        WitDataType::WitInterfaceResource(WitInterfaceResource { 
                            name: class_name.to_string(), 
                            func_defines: func_def_string_list
                        }
                    )
                }
            );
        } else {
            //println!("{}", "Something Wrong!".red());
            return Err(Js2WitConvertErr::WrongFormatErr);
        }
    }
    else if let Some((InterfaceType::Enum, enum_name)) = inter_face_type
    {
        return Ok(WitInterface { 
                    name: enum_name.to_string(),
                    deps_uses: todo!(),
                    inner_data: WitDataType::WitInterfaceEnum(
                        WitInterfaceEnum { 
                            name: enum_name.to_string(),
                            enum_members: class.enum_members.iter().map(|i| i.name.clone()).collect()
                        }
                    )
        });
    }
    // else if let Some((InterfaceType::Interface, _)) = inter_face_type
    // {
    // }
    else
    {
        return Err(Js2WitConvertErr::WrongFormatErr);
    }
}

impl WitDefFile {
    pub fn new(
        target_service: &ApiService,
        service_list: &[ApiService],
    ) -> Self
    {
        for i in &target_service.classes {
            //println!("class name \"{}\"", i.name);

        }

        Self {
            package_name: target_service.service_name,
            package_version: Some("0.1.0-alpha".to_string()),
            deps_uses: (),
            defined_interfaces: (),
            world_section: () 
        }
    }
}

