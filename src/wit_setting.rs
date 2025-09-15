use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WitFileSetting {
    pub version: String,
    pub minimaize: bool, //

    // TODO 以下の引数の型は考える必要がある
    /// 生成するサービス
    pub allowed_service: Vec<String>,
    /// 引数や返り値として扱われるinterfaceを指定する
    /// ```wit
    /// : func(arg1:interface, arg2: interface)-> interface
    /// ```
    pub allowed_interfaces: Vec<String>,
    pub allowed_functions: Vec<String>,  //
    pub copy_function_list: Vec<String>, // resourceに属するメソッドで特にコピーを必要とするような関数(手動で設定する必要がある)
}
