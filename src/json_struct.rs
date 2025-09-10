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
    pub implementing_classes: Vec<String>,
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


pub trait TypeTrait {
    fn get_name(&self) -> String;
    fn get_url(&self) -> Option<String>;
}


// 型情報の取り扱いは返り値も引数も同じ

impl TypeTrait for Type 
{
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_url(&self) -> Option<String> {
        self.url.clone()
    }
}

impl TypeTrait for ReturnType
{
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_url(&self) -> Option<String> {
        self.url.clone()
    }
}

