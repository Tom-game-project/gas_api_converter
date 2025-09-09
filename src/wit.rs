use crate::{get_interface_name_from_js_type, json_struct::{ApiService, Method, Parameter}};
use convert_case::{Case, Casing};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JsTypeString(pub String);
pub struct WitTypeString(pub String);

pub enum Js2WitConvertErr{
    NotPrimitiveType{
        wit_type_string: WitTypeString, 
        unknown_fields:Vec<JsTypeString>
    }, // このエラーは条件付きで正常に復帰可能
    ParameterStringErr,
    ReturnStringErr,
    SyntaxErr,
    WrongFormatErr,
} // Jsの型からWitの変換中に起きるエラーのキャッチ

fn is_ascii_alnum_or_underscore(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn extract_method_name(signature: &str) -> Option<&str> {
    signature.find('(').map(|idx| &signature[..idx])
}

/// 残された可能性がリストの場合のみこの関数を呼び出す
fn wit_dress_list(a: JsTypeString) -> Result<WitTypeString, Js2WitConvertErr>
{
    if a.0.ends_with("[]")
    {
        let b = &a.0[0..a.0.len() - 2];
        let wit_type = 
            convert_wit_type_string(JsTypeString(b.to_string()));

        if let Ok(wit_type_inner) = wit_type
        {
            Ok(WitTypeString(format!("list<{}>", wit_type_inner.0)))
        }
        else if let Err(Js2WitConvertErr::NotPrimitiveType {
            wit_type_string,
            unknown_fields }) = wit_type
        {
            Err(Js2WitConvertErr::NotPrimitiveType {
                wit_type_string: WitTypeString(format!("list<{}>", wit_type_string.0)), 
                unknown_fields: unknown_fields
            }
            )
        }
        else 
        {
            Err(Js2WitConvertErr::SyntaxErr)
        }
    }
    else
    {
        Err(Js2WitConvertErr::SyntaxErr)
    }
}

pub fn convert_wit_type_string(js_type_string: JsTypeString) -> Result<WitTypeString, Js2WitConvertErr>
{
    // primitive type
    match js_type_string.0.as_str() {
        "String" => return Ok(WitTypeString("string".to_string())),
        "Boolean" => return Ok(WitTypeString("boolean".to_string())),
        "Byte" =>  return Ok(WitTypeString("u8".to_string())),
        "Number" => return Ok(WitTypeString("f64".to_string())),
        "Integer" => return Ok(WitTypeString("s64".to_string())),
        "void" => return Ok(WitTypeString("void".to_string())), // 特別
        "Object" => return Ok(WitTypeString("object".to_string())), // 少し考える必要がある部分
        _ => {
            //wit_dress_list(js_type_string.clone())
        }
    }

    // primitive typeでない型,GAS API独自のクラスなど
    if is_ascii_alnum_or_underscore(&js_type_string.0)
    {
        return Err(Js2WitConvertErr::NotPrimitiveType{
                wit_type_string: WitTypeString(
                    js_type_string.0
                        .to_case(Case::Kebab)
                ),
                unknown_fields: vec![js_type_string]
            }
        );
    }

    wit_dress_list(js_type_string.clone())
}

fn wit_convert_arg_type_pair(name:JsTypeString, type_name:JsTypeString) -> Result<WitTypeString, Js2WitConvertErr>
{
    let a = 
        convert_wit_type_string(type_name);
    if let Ok(b) = a
    {
        Ok(WitTypeString(
            format!("{}: {}", 
                name.0, 
                b.0
            )
        ))
    }
    else if let Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) = a
    {
        Err(Js2WitConvertErr::NotPrimitiveType{
                wit_type_string: WitTypeString(
                    format!("{}: {}", 
                        name.0, 
                        wit_type_string.0
                    )
                ),
                unknown_fields
            }
        )
    }
    else 
    {
        return Err(Js2WitConvertErr::ParameterStringErr);
    }
}

/// witの関数宣言の引数部分を作成する
pub fn wit_parameters_string(parameter_list: &Vec<Parameter>) -> Result<WitTypeString, Js2WitConvertErr>
{
    let mut rlist:Vec<String> = vec![];
    let mut unknown_fields_gather = vec![];

    for i in parameter_list 
    {
        let arg_type = 
            wit_convert_arg_type_pair(
                JsTypeString(i.name.clone()), // 引数の名前
                JsTypeString(i.param_type.name.clone()) // タイプの名前
            );

        if let Ok(b) = arg_type{
            rlist.push(b.0);
        }
        else if let Err(Js2WitConvertErr::NotPrimitiveType {
            wit_type_string:b,
            unknown_fields:v }) = arg_type
        {
            rlist.push(b.0);
            unknown_fields_gather = [unknown_fields_gather, v].concat();
        }
        else if let Err(e) = arg_type {
            return Err(e);
        }
    }

    let r_text = WitTypeString(rlist.join(", "));
    if unknown_fields_gather.is_empty()
    {
        Ok(r_text)
    }
    else 
    {
        Err(Js2WitConvertErr::NotPrimitiveType { 
            wit_type_string: r_text,
            unknown_fields: unknown_fields_gather 
        })
    }
}

/// witの関数宣言部分の生成
pub fn wit_gen_func_def(method: &Method) -> Result<WitTypeString, Js2WitConvertErr>
// jsonのmethodnameの記述が`(...)`で終了することを保証してもらう必要がある
{
    let mut unknown_fields_gather = vec![];

    let wit_parameters = 
        wit_parameters_string(&method.parameters);

    let wit_return = 
        convert_wit_type_string(JsTypeString(method.return_type.name.clone()));

    // ======================================== TODO:同じような内容のため後で修正
    let wit_parameters = if let Ok(b) = wit_parameters {
        b
    }
    else if let Err(Js2WitConvertErr::NotPrimitiveType {
        wit_type_string, 
        unknown_fields }) = wit_parameters {
        unknown_fields_gather = [unknown_fields_gather, unknown_fields].concat();
        wit_type_string
    }
    else {
        return Err(Js2WitConvertErr::ParameterStringErr);
    };

    // ========================================
    let wit_return = if let Ok(b) = wit_return
    {
        b
    }
    else if let Err(Js2WitConvertErr::NotPrimitiveType{
        wit_type_string, 
        unknown_fields}) = wit_return
    {
        unknown_fields_gather = [unknown_fields_gather, unknown_fields].concat();
        wit_type_string
    }
    else 
    {
        return Err(Js2WitConvertErr::ReturnStringErr);
    };

    let func_name = 
                extract_method_name(&method.name)
                    .expect("Error: not end with `()`")
                    .to_case(Case::Kebab);
    let func_type = 
                if wit_return.0 == "void" {
                    format!("func ({})",
                        wit_parameters.0,
                    )
                }else{
                    format!("func ({}) -> {}", 
                        wit_parameters.0,
                        wit_return.0
                    )
                };
    let r_text = 
            WitTypeString(
                format!("{}: {}",
                    func_name,
                    func_type
                )
            );

    if unknown_fields_gather.is_empty()
    {
        Ok(r_text)
    }
    else
    {
        Err(
            Js2WitConvertErr::NotPrimitiveType {
                wit_type_string: r_text,
                unknown_fields: unknown_fields_gather 
            }
        )
    }
}

/// interface及びresourceの名前を生成する関数
///
/// `JsTypeString("Class Blob")`のような文字列を受け入れる
///
pub fn wit_gen_interface_name(js_type_name: &JsTypeString) -> Result<WitTypeString, Js2WitConvertErr>
{
    if let Some((_, sliced)) = get_interface_name_from_js_type(js_type_name)
    {
        Ok(
            WitTypeString(
                sliced.to_case(Case::Kebab)
            )
        )
    }
    else 
    {
        Err(Js2WitConvertErr::WrongFormatErr)
    }
}

/// ここから下は、Witファイルを構成するためのプログラム
///

struct WitDefFile
{
    package_name: WitTypeString,
    deps_uses: Vec<WitUseSection>, // use ...; サービスを超えて必要になるinterfaceのimport
    
    defined_interfaces: Vec<WitInterface>,
    world_section: WitWorldSection,
}

/// - Enumの場合
/// interfaceの内部に定義される
/// ```wit
/// interface enum_name {
///     enum enum_name{
///     
///     }
/// }
/// ```
/// - Classの場合
/// リソースになるうるか否かで区別される
///
/// - リソースとして扱わない場合　
/// ```wit
/// interface class_name{
///    ... methods ...
/// }
/// ```
/// - リソースとして扱う場合
/// ```wit
/// interface class_name {
///     resource {
///         ... methods ...
///     }
/// }
/// ```
/// - Interfaceの場合(TODO)
/// Classと同様に処理される
enum WitInterface 
{
    WitInterfaceConst(WitInterfaceConst),
    WitInterfaceResource(WitInterfaceResource),
    WitInterfaceEnum(WitInterfaceEnum)
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値とならないような型
struct WitInterfaceConst
{
    name: WitTypeString,
    deps_uses: Vec<WitUseSection>,
    func_defines:Vec<WitTypeString>,
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値となるような型
struct WitInterfaceResource
{
    name: WitTypeString,
    deps_uses: Vec<WitUseSection>,
    func_defines:Vec<WitTypeString>,
}

struct WitInterfaceEnum
{
    name: WitTypeString,
    enum_members: Vec<WitTypeString>,
}

/// Use文のためのセクション
/// witファイル先頭で使う場合(BeyondService)
/// と、ファイルの中のinterfaceを超えて型を利用する場合(BetondInterface)
/// で使い分ける
enum WitUseSection
{
    BeyondService(WitUseSectionBeyondService),
    BetondInterface(WitUseSectionBetondInterface),
}

struct WitUseSectionBeyondService{
    service:WitTypeString,
    interface: WitTypeString,
}

struct WitUseSectionBetondInterface{
    interface: WitTypeString,
    inners: Vec<WitTypeString>,
}


struct WitWorldSection
{
    imports: Vec<WitTypeString>,
    exports: Vec<WitTypeString>,
}

pub fn wit_gen_package_name(prefix: &str, target_service:&ApiService, wit_version: Option<&WitTypeString>) -> WitTypeString
{
    WitTypeString(
        format!("package {}:{}{}",
            prefix,
            target_service.service_name, 
            if let Some(version) = wit_version{format!("@{}", version.0)} else {"".to_string()}
        )
    )
}

pub fn wit_gen_service_use(prefix: &str, target_service:&JsTypeString, target_interface: &JsTypeString, wit_version: Option<&WitTypeString>) -> Result<WitTypeString, Js2WitConvertErr>
{
    let a = wit_gen_interface_name(
        target_interface
    )?;

    Ok(
        WitTypeString(
            format!("use {}:{}/{}{}",
                prefix,
                target_service.0, 
                a.0,
                if let Some(version) = wit_version{format!("@{}", version.0)} else {"".to_string()}
            )
        )
    )
}

pub fn wit_gen_interface_use(target_resource: &JsTypeString) -> Result<WitTypeString, Js2WitConvertErr>

{
    let a = wit_gen_interface_name(
        target_resource
    )?;

    Ok(WitTypeString(
        format!("use {}.{{{}}}", a.0, a.0)
    ))
}

/*
impl WitDefFile {
    fn constructor(all_service:&[ApiService], target_service: &ApiService)
    {
        let package_name = wit_gen_package_name(
            "gas",
            target_service, 
            Some(&WitTypeString("0.1.0-alpha".to_string())));
        
    }
}
*/


