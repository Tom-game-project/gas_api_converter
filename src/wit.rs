use crate::json_struct::Parameter;

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
