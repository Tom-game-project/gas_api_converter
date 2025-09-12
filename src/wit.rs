use std::fmt::Debug;

use crate::{get_interface_name_from_js_type, json_struct::{ApiService, Method, Parameter}, Type, TypeTrait};
use convert_case::{Case, Casing};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JsTypeString(pub String);
#[derive(Debug)]
pub struct WitTypeString(pub String);

#[derive(Debug)]
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

fn wit_convert_arg_type_pair(parameter: &Parameter) -> Result<WitTypeString, Js2WitConvertErr>
{
    let a = 
        convert_wit_type_string(&parameter.param_type);
    if let Ok(b) = a
    {
        Ok(WitTypeString(
            format!("{}: {}", 
                parameter.name, 
                b.0
            )
        ))
    }
    else if let Err(Js2WitConvertErr::NotPrimitiveType{wit_type_string, unknown_fields}) = a
    {
        Err(Js2WitConvertErr::NotPrimitiveType{
                wit_type_string: WitTypeString(
                    format!("{}: {}", 
                        parameter.name, 
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

fn wit_dress_list<T>(arg: &T) -> Result<WitTypeString, Js2WitConvertErr>
where T:TypeTrait
{
    if arg.get_name().ends_with("[]")
    {
        let b = &arg.get_name()[0..arg.get_name().len() - 2];
        let wit_type = 
            convert_wit_type_string(&Type { name: b.to_string(), url: arg.get_url().clone() });

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

/// 引数をjsonの型にした版
pub fn convert_wit_type_string<T>(arg: &T) -> Result<WitTypeString, Js2WitConvertErr>
where T: TypeTrait
{
    // primitive type
    if arg.get_url().is_none() // 説明のurlが無い -> プリミティブな型を疑う
    {
        match arg.get_name().as_str() {
            "String" => return Ok(WitTypeString("string".to_string())),
            "Boolean" => return Ok(WitTypeString("boolean".to_string())),
            "Byte" =>  return Ok(WitTypeString("u8".to_string())),
            "Number" => return Ok(WitTypeString("f64".to_string())),
            "Integer" => return Ok(WitTypeString("s64".to_string())),
            "void" => return Ok(WitTypeString("void".to_string())), // 特別
            "Object" => return Ok(WitTypeString("object".to_string())), // 少し考える必要がある部分
            "Date" => return Ok(WitTypeString("date".to_string())),
            _ => {
            }
        }
    }

    // primitive typeでない型,GAS API独自のクラスなど
    if is_ascii_alnum_or_underscore(&arg.get_name())
    {
        return Err(Js2WitConvertErr::NotPrimitiveType{
                wit_type_string: WitTypeString(
                    arg.get_name()
                        .to_case(Case::Kebab)
                ),
                unknown_fields: vec![JsTypeString(arg.get_name().clone())]
            }
        );
    }

    wit_dress_list(arg)
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
                i
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

    let wit_return = convert_wit_type_string(
        &method.return_type
    );
    //convert_wit_type_string(JsTypeString(method.return_type.name.clone()));

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

