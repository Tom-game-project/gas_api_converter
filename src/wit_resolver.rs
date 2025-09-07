use crate::{
    json_struct::{ApiService, Class}, 
    wit::JsTypeString
};

#[derive(Debug)]
pub struct TypeDefineLocation{
    pub service: JsTypeString,
    pub class: JsTypeString
}

pub fn find_type_define_location(
    self_service_set: &[ApiService],
    js_type: &JsTypeString
) -> Option<TypeDefineLocation>
{
    self_service_set
        .iter()
        .find_map(
        |i| 
        i.classes.iter().find_map(
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

pub fn is_self_type(self_class: &Class, js_type: &JsTypeString) -> bool
{
    if self_class.name.starts_with("Class")
    {
        let sliced = &self_class.name[6..];
        sliced == js_type.0
    }
    else if self_class.name.starts_with("Enum")
    {
        let sliced = &self_class.name[5..];
        sliced == js_type.0
    }
    else if self_class.name.starts_with("Interface")
    {
        let sliced = &self_class.name[10..];
        sliced == js_type.0
    }
    else {
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

