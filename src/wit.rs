use crate::json_struct::{Method, Parameter};
use convert_case::{Case, Casing};

#[derive(Clone, Debug)]
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
pub fn wit_parameters_string(parameter_list: Vec<Parameter>) -> Result<WitTypeString, Js2WitConvertErr>
{
    let mut rlist:Vec<String> = vec![];
    let mut unknown_fields_gather = vec![];

    for i in parameter_list 
    {
        let arg_type = 
            wit_convert_arg_type_pair(
                JsTypeString(i.name), // 引数の名前
                JsTypeString(i.param_type.name) // タイプの名前
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
pub fn wit_gen_func_def(method: Method) -> Result<WitTypeString, Js2WitConvertErr>
// jsonのmethodnameの記述が`(...)`で終了することを保証してもらう必要がある
{
    let mut unknown_fields_gather = vec![];

    let wit_parameters = 
        wit_parameters_string(method.parameters);

    let wit_return = 
        convert_wit_type_string(JsTypeString(method.return_type.name));

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
        Ok(
            r_text
        )
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

