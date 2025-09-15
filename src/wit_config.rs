use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WitFileConfig {
    pub version: String,
    pub minimaize: bool, //

    // TODO 以下の引数の型は考える必要がある
    /// 生成するサービス
    pub allowed_service: Vec<String>,
    /// 引数や返り値として扱われるinterfaceを指定する
    /// Vecに属していない型を使用する関数は無視される
    /// ```wit
    /// : func(arg1:interface, arg2: interface)-> interface
    /// ```
    pub allowed_interfaces: Vec<String>,
    pub allowed_functions: Vec<String>,  //
    pub copy_function_list: Vec<String>, // resourceに属するメソッドで特にコピーを必要とするような関数(手動で設定する必要がある)
}

// TODO
// 以下のようなGASのAPIから形式的には読み取れない情報を手動で設定するためのConfig構造体が必要
// - nullを返却する可能性(返り値の型がoptionになるかどうか)
// - errを返却する可能性(エラー)
//
// 以上の情報さえわかればjsラッパーやwitファイルは自動的に生成が可能になる
//
//
