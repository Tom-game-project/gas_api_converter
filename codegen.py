
def codegen_js00(class_name:str, methods:list[str]):
    code: str = f"""
export const {class_name} = {class_name} {{
{"".join(methods)}
}};
"""
    return code


def wit_builder_interface(
        intereface_name:str,
        use_defines:list[str],
        interface_members:list[str]
):
    use_section = ""
    if use_defines:
        use_section += "use "
        #use_section += "".join()
    return f"""
interface {intereface_name}{{
    {use_section}
    {"".join(intereface_name)}
}}
"""

def wit_builder_func(
        func_name:str,
        param_types: list[str],
        return_type: str
):
    exp_args_string = ','.join(param_types)
    if return_type == "":
        return f"{func_name}: func({exp_args_string});\n"
    else:
        return f"{func_name}: func({exp_args_string}) -> {return_type};\n"


def js_builder_constructor():
    value_obj = "obj"
    return f"""
        constructor({value_obj})
        {{
            this.obj = {value_obj};
        }}
"""


def js_builder_call_new(
    func_name:str = "func",
    args_name:list[str] = ["id"],
    return_class:str = "T",
    parent_class:str = "DriveApp",
):

    exp_args_string = ','.join(args_name)
    init_func_call = f"{parent_class}.{func_name}({exp_args_string})";
    return f"""
        {func_name} ({exp_args_string}){{
            const a = {init_func_call};
            return new {return_class}(a);
        }}
"""


def js_builder_chainable_func(
    func_name:str = "func",
    args_name:list[str] = ["id"]
):

    exp_args_string = ','.join(args_name)
    return f"""
        {func_name} ({exp_args_string}){{
            this.obj.{func_name}({exp_args_string});
            return this;
        }}
"""


def js_builder_return_it_as_is(
    func_name:str = "func",
    args_name:list[str] = ["id"]
):
    """
    primitive typeを返却するようなGASの関数はこれを使用する
    """
    exp_args_string = ','.join(args_name)
    return """
        {func_name} ({exp_args_string}) {{
            const r = this.obj.{func_name}({exp_args_string});
            return r;
        }}
"""

if __name__ == "__main__":
    print(codegen_js00("GasSpreadSheetApp", [
        """
        constructor()
        {

        }"""
        ,
        """
        getId()
        {

        }"""
    ]))
