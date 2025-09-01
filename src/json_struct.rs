use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiService {
    pub service_name: String,
    pub url: String,
    pub classes: Vec<Class>,
}

#[derive(Debug, Deserialize)]
pub struct Class {
    pub name: String,
    pub url: String,
    pub description: String,
    pub methods: Vec<Method>,
    pub enum_members: Vec<EnumMember>,
}

#[derive(Debug, Deserialize)]
pub struct Method {
    pub name: String,
    pub description: String,
    pub return_type: ReturnType,
    pub parameters: Vec<Parameter>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: Type,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Type {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReturnType {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct EnumMember {
    pub name: String,
    pub description: String,
}
