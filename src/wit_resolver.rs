use crate::{ json_struct::{ApiService, Class}, 
    wit::JsTypeString
};

#[derive(Debug)]
pub struct TypeDefineLocation
{
    pub service: JsTypeString,
    pub class: JsTypeString
}

/// 型が定義されている場所を探し返す関数を追加
/// 
pub fn find_type_define_location(
    self_service_set: &[ApiService],
    js_type: &JsTypeString
) -> Option<TypeDefineLocation>
{
    self_service_set
        .iter()
        .find_map(
        |i| 
        i.classes
            .iter()
            .find_map(
                |j| 
                if is_self_type(j, js_type) {
                    Some(
                        TypeDefineLocation {
                            service: JsTypeString(i.service_name.clone()),
                            class: JsTypeString(j.name.clone())
                        }
                    )
                } else {
                    None
                }
        )
    )
}

pub enum InterfaceType
{
    Interface,
    Class,
    Enum,
}

/// 決まったフォーマットで書かれているinterface相当のデータ構造の名前を取得
pub fn get_interface_name_from_js_type(js_type: &JsTypeString) -> Option<(InterfaceType, &str)>
{
    if js_type.0.starts_with("Class")
    {
        let sliced = &js_type.0[6..];
        Some((InterfaceType::Class, sliced))
    }
    else if js_type.0.starts_with("Enum")
    {
        let sliced = &js_type.0[5..];
        Some((InterfaceType::Enum, sliced))
    }
    else if js_type.0.starts_with("Interface")
    {
        let sliced = &js_type.0[10..];
        Some((InterfaceType:: Interface, sliced))
    }
    else 
    {
        None
    }
}

pub fn is_self_type(self_class: &Class, js_type: &JsTypeString) -> bool
{
    if let Some((_, sliced)) = get_interface_name_from_js_type(
        &JsTypeString(
            self_class.name.clone()
        )
    )
    {
        sliced == js_type.0
    }
    else 
    {
        false
    }
}

pub fn is_in_same_service(
    self_service: &ApiService,
    js_type: &JsTypeString
) -> bool
{
    self_service.classes.iter().any(
        |a| is_self_type(a, js_type)
    )
}

pub fn is_in_somewhere_service(
    self_service_set: &[ApiService],
    js_type: &JsTypeString
) -> bool
{
    self_service_set.iter().any(
        |service| is_in_same_service(service, js_type)
    )
}

// リソースは、自分自身を含めた他のクラスのメソッドのどれかに、
// 自分自身のタイプを引数にとったり、戻り値をとったりするメソッドを一つでも見つけられる
//
// その上で、すでに返り値となっているようなクラスは以下のようなアクセスが可能。
//
// ```
// use interface.{resource, ...};
// ```

