
use crate::json_struct::{Method, Parameter};
use convert_case::{Case, Casing};

#[derive(Clone)]
pub struct JsTypeString(pub String);
pub struct WitTypeString(pub String);

pub enum Js2WitConvertErr{
    NotPrimitiveType(WitTypeString), // このエラーは条件付きで正常に復帰可能
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

        match convert_wit_type_string(JsTypeString(b.to_string())) {
            Ok(wit_type) => {
                return Ok(WitTypeString(format!("list<{}>", wit_type.0)))
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    else
    {
        //convert_wit_type_string(a) // ここで再帰するとoverflowを起こす
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
        "Number" => return Ok(WitTypeString("s64".to_string())),
        _ => {
            //wit_dress_list(js_type_string.clone())
        }
    }

    // primitive typeでない型,GAS API独自のクラスなど
    if is_ascii_alnum_or_underscore(&js_type_string.0)
    {
        return Err(Js2WitConvertErr::NotPrimitiveType(WitTypeString(
            js_type_string.0.to_case(Case::Kebab)
        )));
    }

    wit_dress_list(js_type_string.clone())
}

pub fn parameters_string(parameter_list: Vec<Parameter>) -> String
{
    let mut rlist:Vec<String> = vec![];
    for i in parameter_list {
        rlist.push(
            format!("{}: {}", i.name, i.param_type.name)
        );
    }
    rlist.join(", ")
}


/// witの関数宣言部分の生成
pub fn wit_gen_func_def(method: Method) -> String 
// jsonのmethodnameの記述が`()`で終了することを保証してもらう必要がある
{
    format!("   {}: {}",
        extract_method_name(&method.name)
            .expect("Error: not end with `()`")
            .to_case(Case::Kebab),
        format!("func ({}) -> {}", 
        parameters_string(method.parameters),
        method.return_type.name))
}

/// ある関数が依存する型(interface)を返す
///
pub fn dep_interface()
{
    //
}
