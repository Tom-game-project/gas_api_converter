/// jsonに基づいてwit構造体に格納,witファイルとして出力するプログラム
///

use crate::{find_type_define_location, get_interface_name_from_js_type, wit_gen_func_def, Class, Conv2JsTypeString, InterfaceType, Js2WitConvertErr, Method, TypeDefineLocation, WitTypeString};
use crate::{ json_struct::{ApiService}, 
    wit::JsTypeString
};
use std::collections::HashMap;
use std::{collections::HashSet};
use convert_case::{Case, Casing};

use owo_colors::OwoColorize;

#[derive(Debug)]
pub struct WitDefFile
{
    pub package_name: String,
    pub package_version: Option<String>,
    pub deps_uses: Vec<String>, // use ...; サービスを超えて必要になるinterfaceのimport
    pub defined_interfaces: Vec<WitInterface>,
    pub world_section: WitWorldSection,
}

#[derive(Debug)]
pub struct WitInterface
{
    pub name: String,
    /// (example: [blob, file])
    pub deps_uses: Vec<TypeRequirements>, // 必要とされる型
    pub inner_data: WitDataType,
}

#[derive(Debug)]
pub enum WitDataType
{
    /// 自分自身が要素として返却されないクラス
    WitInterfaceConst(WitInterfaceConst),
    /// 自分自身が要素として返却されるクラス
    WitInterfaceResource(WitInterfaceResource),
    /// 列挙型
    WitInterfaceEnum(WitInterfaceEnum)
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値とならないような型
#[derive(Debug)]
pub struct WitInterfaceConst
{
    pub name: String,
    pub func_defines:Vec<String>,
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値となるような型
#[derive(Debug)]
pub struct WitInterfaceResource
{
    pub name: String,
    pub func_defines:Vec<String>,
}

#[derive(Debug)]
pub struct WitInterfaceEnum
{
    pub name: String,
    pub enum_members: Vec<String>,
}

/// Use文のためのセクション
/// witファイル先頭で使う場合(BeyondService)
/// と、ファイルの中のinterfaceを超えて型を利用する場合(BetondInterface)
/// で使い分ける

#[derive(Debug)]
pub struct WitWorldSection
{
    pub imports: Vec<String>,
    pub exports: Vec<String>,
}

fn required_find_type_define_location<'a, T>(
    self_service_set: &[ApiService],  // すべてのサービスを格納したリストなど
    deps_uses: T                      // unknown_fieldsから返されたJsTypeStringのリストなど
) -> Vec<TypeDefineLocation>
where 
    T: IntoIterator<Item = &'a TypeRequirements>,
{
    let mut rlist = vec![];
    for i in deps_uses{
        //println!("{:?}", i.blue());
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


fn get_func_def_string_vec_with_filter(
    methods: &[Method],
    allowed_interfaces: &[JsTypeString]
) -> Result<(Vec<WitFuncDef>, HashSet<JsTypeString>), Js2WitConvertErr>
{
    let mut rlist = vec![];                       // メソッドの定義をwit文字列にしたもの
    let mut deps_uses: HashSet<JsTypeString> = HashSet::new(); // メソッド集合を定義するクラスが必要とする型
    for j in methods {
        match wit_gen_func_def(j){
            Ok(b) => {
                rlist.push(
                    b.0.conv2_wit_func_def()
                );
            }
            Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields: unknown_interface}) => {
                // 許容された型以外が引数または返り値に含まれていた場合
                // 追加しない
                if unknown_interface
                    .iter()
                    .all(|i| allowed_interfaces.contains(i))
                {
                    rlist.push(wit_type_string.conv2_wit_func_def());
                    deps_uses.extend(unknown_interface);
                }
                else
                {
                    // println!("{:?}", unknown_interface.red());
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok((rlist, deps_uses))
}



/// witのmethodとして解釈できる文字列を格納する
#[derive(Clone)]
struct WitFuncDef(String);

// TODO 本来であれば変換のたびに検証をしたい
trait Conv2WitFuncDef {
    fn conv2_wit_func_def(self) -> WitFuncDef;
}

impl Conv2WitFuncDef for String {
    fn conv2_wit_func_def(self) -> WitFuncDef 
    {
        WitFuncDef(self)
    }
}

impl Conv2WitFuncDef for WitTypeString 
{
    fn conv2_wit_func_def(self) -> WitFuncDef 
    {
        WitFuncDef(self.0)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TypeRequirements(pub String);

trait Conv2TypeRequirements {
    fn conv2_type_requirements(self) -> TypeRequirements;
}

impl Conv2TypeRequirements for JsTypeString
{
    fn conv2_type_requirements(self) -> TypeRequirements {
        TypeRequirements(self.0)
    }
}

/// prefixがついたままのクラス、インターフェイス、列挙型の宣言
#[derive(Hash, PartialEq, Eq)]
struct JsClassName(String);

trait Conv2ClassName {
    fn conv2_class_name(self) -> JsClassName;
}

impl Conv2ClassName for String {
    fn conv2_class_name(self) -> JsClassName {
        JsClassName(self)
    }
}

impl Conv2ClassName for JsTypeString {
    fn conv2_class_name(self) -> JsClassName {
        JsClassName(self.0)
    }
}

impl Conv2JsTypeString for TypeRequirements
{
    fn conv2_js_type_string(self) -> JsTypeString {
        JsTypeString(self.0)
    }
}

impl Conv2JsTypeString for String
{
    fn conv2_js_type_string(self) -> JsTypeString {
        JsTypeString(self)
    }
}

/// サービス内に存在するメソッドのうち条件に当てはまるものを
fn generate_wit_methods_in_service_with_filter(
    class_list:&[Class], // サービス内に存在するすべてのinterface
    allowed_interfaces: &[JsTypeString]
) -> Result<
    HashMap<
        JsClassName,
        (
            Vec<WitFuncDef>,
            HashSet<TypeRequirements>
        )>,
    Js2WitConvertErr>
{
    let mut class_methods_dir = HashMap::new();
    for class in class_list {
        let b = get_func_def_string_vec_with_filter(&class.methods, allowed_interfaces);
        if let Ok((i0, i1)) = b
        {
            let typed_list = i0;
            let type_requirements = i1
                .iter()
                .map(|i| i.clone().conv2_type_requirements())
                .collect();
            class_methods_dir.insert(
                class.name.clone().conv2_class_name(),
                (typed_list, type_requirements));
        }
        else if let Err(e) = b
        {
            return Err(e);
        }
    }
    Ok(class_methods_dir)
}

/// 他のクラスから自身（classに渡した引数）が利用されるかを調べる関数　
/// クラスの名前が正しい方法で記述されていない場合はパニックを起こす
fn is_used_by_others(class: &Class, type_requirements_list: &HashSet<&TypeRequirements>) -> bool
{
    let js_type_string = JsTypeString(class.name.clone());
    let js_type = get_interface_name_from_js_type(&js_type_string);
    let (_, j) = js_type.unwrap();
    type_requirements_list.contains(&TypeRequirements(j.to_string()))
}


/// wit定義を表現する構造体を生成する
/// 指定されたinterfaceと、それに関連するメソッドのみの出力
pub fn generate_wit_definition_with_filter(
    target_service: &ApiService,
    api_services :&[ApiService],
    allowed_interfaces: &[JsTypeString],
) -> Result<WitDefFile, Js2WitConvertErr>
{
    let all_method 
        = generate_wit_methods_in_service_with_filter(
            &target_service.classes,
            allowed_interfaces)?;
    let ts 
        = all_method.values().flat_map(|(_, hs)| hs.iter()).collect();

    let mut defined_interfaces = vec![];
    // サービス内のすべてのクラス
    for class in &target_service.classes {
        // クラスごとに処理して
        if let Ok(wit_interface) = 
            generate_wit_interface(
                class, 
                &all_method,
                &ts
            )
        {
            if allowed_interfaces.contains(
                &JsTypeString(
                    get_interface_name_from_js_type(
                        &class.name
                        .clone()
                        .conv2_js_type_string()
                    )
                    .unwrap()
                    .1
                    .to_string()
                )
            )
            {
                defined_interfaces.push(wit_interface);
            }
            else
            {
                // 対応していないクラス
                // println!("{}", class.name);
            }
        }
        else
        {
        }
    }

    // サービスで必要となるもの
    let deps_uses 
        = required_find_type_define_location(
            api_services,
            ts
        )
        .iter()
        .filter(|i| i.service.0 != target_service.service_name) // サービスを超えてimportするもの
        .map(|i|{
            let c = &i.class.0.clone().conv2_js_type_string();
            let (_, a) = get_interface_name_from_js_type(
                c
            ).unwrap();
            let aa = a.to_case(Case::Kebab);
            format!("use gas:{}/{}@0.1.0-alpha", i.service.0, aa)
        }
        )
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();

    Ok(WitDefFile { 
        package_name: target_service.service_name.clone(),
        package_version: Some("0.1.0-alpha".to_string()),
        deps_uses,
        defined_interfaces,
        world_section: WitWorldSection { imports: vec![], exports: vec![] }
    })
}

fn generate_wit_interface(
    class: &Class,
    class_hashmap: &HashMap<
        JsClassName,
        (
            Vec<WitFuncDef>,          // witの関数定義リスト
            HashSet<TypeRequirements> // クラスが必要とする型
        )>,
    service_requirements: &HashSet<&TypeRequirements>
) -> Result<WitInterface, Js2WitConvertErr>
{
    let js_class_name = class.name.clone().conv2_js_type_string();

    if let Some((
        _wit_class_type,
        wit_class_name)
    ) = get_interface_name_from_js_type(
        &js_class_name
    )
    {
        let required_type_list: Vec<TypeRequirements> 
            = class_hashmap
                .get(&js_class_name.clone().conv2_class_name())
                .unwrap()
                .1
                .clone()
                .into_iter()
                .collect();
        Ok(WitInterface{
            name: wit_class_name.to_string(),
            deps_uses: required_type_list,
            inner_data: generate_wit_data_type(class, class_hashmap, service_requirements)?
        })
    }
    else 
    {
        Err(Js2WitConvertErr::WrongFormatErr)
    }
}

/// class interface enumの単位の出力
fn generate_wit_data_type(
    class: &Class,
    class_hashmap:
    &HashMap<
        JsClassName,
        (
            Vec<WitFuncDef>,
            HashSet<TypeRequirements>
        )>,
    service_requirements: &HashSet<&TypeRequirements>
) -> Result<WitDataType, Js2WitConvertErr>
{
    let js_class_name = class.name.clone().conv2_js_type_string();

    if let Some((
        wit_class_type,
        wit_class_name)
    ) = get_interface_name_from_js_type(
        &js_class_name
    )
    {
        match wit_class_type {
            InterfaceType::Class => {
                let func_defines =  
                    class_hashmap
                        .get(&js_class_name.clone().conv2_class_name())
                        .unwrap()
                        .0
                        .iter()
                        .map(|i|i.0.clone())
                        .collect();
                if is_used_by_others(
                    class, 
                    &service_requirements)
                {
                    // resource
                    Ok(WitDataType::WitInterfaceResource(WitInterfaceResource {
                        name: wit_class_name.to_string(),
                        func_defines
                    }))
                }
                else
                {
                    // const
                    Ok(WitDataType::WitInterfaceConst(WitInterfaceConst {
                        name: wit_class_name.to_string(),
                        func_defines
                    }))
                }
            }
            InterfaceType::Interface => {
                // resource <- 他の型の返り値になることが前提とされているはずなので
                let func_defines 
                    = class_hashmap
                        .get(&js_class_name.clone().conv2_class_name())
                        .unwrap()
                        .0
                        .iter()
                        .map(|i|i.0.clone())
                        .collect();
                Ok(WitDataType::WitInterfaceResource(
                        WitInterfaceResource { 
                            name: wit_class_name.to_string(), 
                            func_defines
                }))
            }
            InterfaceType::Enum => {
                Ok(WitDataType::WitInterfaceEnum(WitInterfaceEnum { 
                        name: wit_class_name.to_string(),
                        enum_members: class.enum_members.iter().map(|i| i.name.clone()).collect()
                }))
            }
        }
    }
    else 
    {
        Err(Js2WitConvertErr::WrongFormatErr)
    }

}

/*
impl WitDefFile {
    /// WITファイルを出力するために必要な構造体の初期化
    pub fn new(
        target_service: &ApiService,
        service_list: &[ApiService],
    ) -> Self
    {
        let mut out_service = vec![];
        let mut wit_interface_list = vec![];

        for i in &target_service.classes {
            //if let Ok((j, k))= generate_wit_data_type(
            //    i, target_service, service_list
            //)
            //{
            //    out_service = [out_service, k].concat();
            //    wit_interface_list.push(j);
            //}
            //else
            //{
            //    // TODO: Errorが起きて変換が出来ないclassがあった場合それを通知する方法を考える
            //}
        }

        Self {
            package_name: target_service.service_name.clone(),
            package_version: Some("0.1.0-alpha".to_string()),
            deps_uses: out_service
                .iter()
                .map(|i|{
                    let c = 
                        &JsTypeString(i.class.0.to_string());
                    let (_, a) = get_interface_name_from_js_type(
                        c
                    ).unwrap();
                    let aa = a.to_case(Case::Kebab);
                    format!("use gas:{}/{}@0.1.0-alpha", i.service.0, aa)
                }
                )
                .collect::<HashSet<String>>()
                .into_iter()
                .collect(),
            defined_interfaces: wit_interface_list,
            world_section: WitWorldSection { imports: vec![], exports: vec![] }
        }
    }
}
*/
