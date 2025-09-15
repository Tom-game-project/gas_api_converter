use crate::{TypeRequirements, WitDataType, WitDefFile, WitInterface};
use convert_case::{Case, Casing};

/// WitDefFileに基づいてwit formatに従った文字列を出力する
pub fn generate_wit_definition_string(wit_def_file: &WitDefFile) -> String {
    let mut rlist = vec![];

    rlist.push(format!(
        "package gas:{}{};\n",
        wit_def_file.package_name, // サービス(パッケージ)の名前
        wit_def_file
            .package_version // サービス(サービス)のバージョン
            .clone()
            .map_or(String::new(), |i| format!("@{i}"))
    ));

    // ==== useセクション(サービス全体で必要となるインターフェイスのimport) ====
    for i in &wit_def_file.deps_uses {
        rlist.push(format!("{i};"));
    }

    for i in &wit_def_file.defined_interfaces {
        rlist.push(generate_wit_interface_string(i));
    }
    rlist.join("\n")
}

fn generate_wit_interface_string(wit_interface: &WitInterface) -> String {
    // wit_interface.name
    let rstring = format!(
        "interface {} {{
{}
{}
}}
",
        wit_interface.name.to_case(Case::Kebab),
        generate_wit_uses(&wit_interface.deps_uses)
            .iter()
            .map(|i| format!("    {i};\n"))
            .collect::<String>(),
        generate_wit_inner_struct(&wit_interface.inner_data),
    );
    rstring
}

/// witを生成する
fn generate_wit_uses(deps_uses: &[TypeRequirements]) -> Vec<String> {
    deps_uses
        .iter()
        .map(|i| {
            let aa = i.0.to_case(Case::Kebab);
            format!("use {aa}.{{{aa}}}")
        })
        .collect()
}

fn generate_wit_inner_struct(wit_data_type: &WitDataType) -> String {
    if let WitDataType::WitInterfaceConst(inner) = wit_data_type {
        format!(
            "
{}
",
            inner
                .func_defines
                .iter()
                .map(|i| format!("    {i};\n"))
                .collect::<String>()
        )
    } else if let WitDataType::WitInterfaceEnum(inner) = wit_data_type {
        format!(
            "
enum {} {{
{}
}}
",
            inner.name.to_case(Case::Kebab),
            inner
                .enum_members
                .iter()
                .map(|i| format!("        {},\n", i.to_case(Case::Kebab)))
                .collect::<String>()
        )
    } else if let WitDataType::WitInterfaceResource(inner) = wit_data_type {
        format!(
            "
resource {} {{
{}
}}
",
            inner.name.to_case(Case::Kebab),
            inner
                .func_defines
                .iter()
                .map(|i| format!("        {i};\n"))
                .collect::<String>()
        )
    } else {
        unreachable!()
    }
}
