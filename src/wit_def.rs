use crate::wit::WitTypeString;

/// ここから下は、Witファイルを構成するためのプログラム
///

struct WitDefFile
{
    package_name: String,
    package_version: Option<String>,
    deps_uses: Vec<String>, // use ...; サービスを超えて必要になるinterfaceのimport
    defined_interfaces: Vec<WitInterface>,
    world_section: WitWorldSection,
}

struct WitInterface
{
    name: String,
    /// (example: blob, file)
    deps_uses: Vec<String>, 
    inner_data: WitDataType,
}

enum WitDataType
{
    /// 自分自身が要素として返却されないクラス
    WitInterfaceConst(WitInterfaceConst),
    /// 自分自身が要素として返却されるクラス
    WitInterfaceResource(WitInterfaceResource),
    /// 列挙型
    WitInterfaceEnum(WitInterfaceEnum)
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値とならないような型
struct WitInterfaceConst
{
    name: String,
    func_defines:Vec<String>,
}

// 自分自身が自分を含めた他のクラスのメソッドの返り値となるような型
struct WitInterfaceResource
{
    name: String,
    func_defines:Vec<String>,
}

struct WitInterfaceEnum
{
    name: String,
    enum_members: Vec<String>,
}

/// Use文のためのセクション
/// witファイル先頭で使う場合(BeyondService)
/// と、ファイルの中のinterfaceを超えて型を利用する場合(BetondInterface)
/// で使い分ける

struct WitWorldSection
{
    imports: Vec<String>,
    exports: Vec<String>,
}


/// witを生成する
fn generate_wit_definition(wit_def_file: &WitDefFile)
{
    let mut rlist = vec![];

    rlist.push(format!("{}{};",
            wit_def_file.package_name,   // サービス(パッケージ)の名前
            wit_def_file.package_version // サービス(サービス)のバージョン
                .clone()
                .map_or(String::new(), |i| format!("@{}", i))
    ));

    // ==== useセクション(サービス全体で必要となるインターフェイスのimport) ====
    for i in &wit_def_file.deps_uses{
        rlist.push(format!("{};", i));
    }

    for i in &wit_def_file.defined_interfaces
    {
        rlist.push(generate_wit_interface_string(i));
    }
    return ;
}

fn generate_wit_uses(deps_uses: &[String]) -> Vec<String>
{
    deps_uses
        .iter()
        .map(|i| format!("use {}.{{{}}}", i, i))
        .collect()
}

fn generate_wit_interface_string(wit_interface: &WitInterface) -> String
{
    // wit_interface.name
    let rstring = format!(
"{} {{
{}
{}
}}",
    wit_interface.name,
    generate_wit_uses(&wit_interface.deps_uses)
    .iter()
    .map(|i| format!("    {};",i))
    .collect::<String>(),
generate_wit_inner_struct(&wit_interface.inner_data), // TODO : wit_interface.inner_data
);
    rstring
}

fn generate_wit_inner_struct(wit_data_type: &WitDataType) -> String
{
    match wit_data_type {
        WitDataType::WitInterfaceConst(inner) => {
            format!("
    interface {} {{
{}
    }}",
    inner.name, 
    inner.func_defines
        .iter()
        .map(|i| format!("        {};", i))
        .collect::<String>())
        }
        WitDataType::WitInterfaceEnum(inner) => {
            format!("
    interface {} {{
        enum {} {{
{}
        }}
    }}",
    inner.name, 
    inner.name, 
    inner.enum_members
        .iter()
        .map(|i| format!("            {};", i))
        .collect::<String>())
        }
        WitDataType::WitInterfaceResource(inner) => {
            format!("
    interface {} {{
        resource {} {{
{}
        }}
    }}",
    inner.name, 
    inner.name, 
    inner.func_defines
        .iter()
        .map(|i| format!("            {};", i))
        .collect::<String>())
        }
    }
}
