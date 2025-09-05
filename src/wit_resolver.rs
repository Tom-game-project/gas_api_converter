use crate::{
    json_struct::{ApiService, Class}, 
    wit::JsTypeString
};


pub fn is_self_type(self_class: &Class, js_type: &JsTypeString) -> bool
{
    self_class.name == js_type.0
}

pub fn is_in_same_service(
    self_service: &ApiService,
    js_type: &JsTypeString
) -> bool
{
    self_service.classes.iter().any(
        |a| 
        if a.name.starts_with("Class")
        {
            let sliced = &a.name[6..];
            sliced == js_type.0
        }
        else if a.name.starts_with("Enum")
        {
            let sliced = &a.name[5..];
            sliced == js_type.0
        }
        else if a.name.starts_with("Interface")
        {
            let sliced = &a.name[10..];
            sliced == js_type.0
        }
        else {
            false
        }
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
