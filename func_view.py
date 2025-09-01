"""
ラッパープログラム生成用プログラム

## 欲しい機能

- 自分で定義したい関数は無視(自分で定義したものに置き換え)

- ワールドをまたぐ型を使用した関数を生成するか否かオプショナルにする

- 

---

## 独自に定義した用語集


### クラスに対して

- chainable(class): メソッドチェーン可能(T/F)

- is_origin(class): 自分自身を返却するメソッドを持たないクラス

### メソッドに対して



### codegen

#### wit

- クラスに属するメソッドの
"""



import json 
import sys
import os
import pprint


def get_all_classes(file_path: str) -> list[str]:
    rlist = []
    with open(file_path, 'r', encoding='utf-8') as f:
        service_data = json.load(f)
    for class_info in service_data.get("classes", []):
        class_name = class_info.get('name', 'N/A')
        rlist.append(class_name)
    return rlist


def is_in_classes_list(class_list:list[str], type_name:str) -> bool:
    for i in class_list:
        t_name:str
        if i.startswith("Class"):
            t_name = i[6:]
        elif i.startswith("Enum"):
            t_name = i[5:]
        else:
            print("Error")
            return False
        if t_name == type_name:
            return True
    return False

def eq_class(class_name: str,type_name: str) -> bool:
    if type_name.startswith("Class"):
        t_name = type_name[6:]

    elif type_name.startswith("Enum"):
        t_name = type_name[5:]
    else:
        return False
    return t_name == class_name



def is_primitive_type(type_name: str) -> bool:
    primitive_type_list = [
            "String",
            "Integer",
            "Number",
            "Boolean",
            "Object",
            "Date",
            "void",
    ]
    return type_name in primitive_type_list


def is_known_type(class_list: list[str], type_name: str) -> bool:
    """
    名前空間の中で定義されたタイプのみを利用した関数かどうかを調べる
    """

    while type_name.endswith("[]"):
        type_name = type_name[:-2]
    return not (is_in_classes_list(class_list, type_name) or  is_primitive_type(type_name))



def display_digest_only_unexpected(file_path):
    """
    ワールドをまたぐ型を使用した関数を表示する
    """
    all_classes = get_all_classes(file_path) # すべてのクラスを取得する
    """指定されたJSONファイルを読み込み、内容を要約して表示する"""
    if not os.path.exists(file_path):
        print(f"Error: File not found at '{file_path}'")
        return

    with open(file_path, 'r', encoding='utf-8') as f:
        service_data = json.load(f)

    if not service_data or "service_name" not in service_data:
        print("Error: JSON format is not as expected.")
        return

    print(f"--- Service: {service_data.get('service_name', 'N/A')} ---")
    print(f"URL: {service_data.get('url', 'N/A')}\n")

    for class_info in service_data.get("classes", []):
        class_name = class_info.get('name', 'N/A')
        print(f"Class: {class_name}")
        class_description = class_info.get('description')
        if class_description:
            print(f"  Description: {class_description}")

        # メソッド情報を表示
        if class_info.get("methods"):
            for method in class_info.get("methods", []):
                class_not_expected = False
                params_list = []
                for param in method.get("parameters", []):
                    param_type = param.get('type', {}).get('name', 'any')
                    class_not_expected |= is_known_type(all_classes, param_type)
                    params_list.append(f"{param.get('name', '?')}: {param_type}")
                
                params_str = ", ".join(params_list)
                return_type = method.get('return_type', {}).get('name', 'void')
                class_not_expected |= is_known_type(all_classes, return_type)
                method_name = method.get('name', '').split('(')[0]

                if class_not_expected:
                    print(f"  - {method_name}({params_str}) -> {return_type}")
                else:
                    #print("Ok")
                    pass

        # Enumメンバー情報を表示
        if class_info.get("enum_members"):
            print("  Enum Members:")
            for member in class_info.get("enum_members", []):
                print(f"    - {member.get('name', '?')}: {member.get('description', '')}")

        print("\n") # クラスごとに改行

def display_digest_only_origin_class(file_path):
    """
    自分自身を返却するメソッドを持たないクラス   
    """
    all_classes = get_all_classes(file_path) # すべてのクラスを取得する
    """指定されたJSONファイルを読み込み、内容を要約して表示する"""
    if not os.path.exists(file_path):
        print(f"Error: File not found at '{file_path}'")
        return

    with open(file_path, 'r', encoding='utf-8') as f:
        service_data = json.load(f)

    if not service_data or "service_name" not in service_data:
        print("Error: JSON format is not as expected.")
        return

    print(f"--- Service: {service_data.get('service_name', 'N/A')} ---")
    print(f"URL: {service_data.get('url', 'N/A')}\n")

    for class_info in service_data.get("classes", []):
        class_name = class_info.get('name', 'N/A')

        # メソッド情報を表示
        have_self_arrow = False
        if class_info.get("methods"):
            for method in class_info.get("methods", []):
                params_list = []
                for param in method.get("parameters", []):
                    param_type = param.get('type', {}).get('name', 'any')
                    params_list.append(f"{param.get('name', '?')}: {param_type}")
                
                params_str = ", ".join(params_list)
                return_type = method.get('return_type', {}).get('name', 'void')
                method_name = method.get('name', '').split('(')[0]
                if eq_class(return_type, class_name):
                    have_self_arrow |= True
                    #print(f"{class_name} {member.get('name', '?')}: {member.get('description', '')}")

        if not have_self_arrow:
            print(f"Class: {class_name}")
            class_description = class_info.get('description')
            if class_description:
                print(f"  Description: {class_description}")

        # Enumメンバー情報を表示
        if class_info.get("enum_members"):
            print("  Enum Members:")
            for member in class_info.get("enum_members", []):
                print(f"    - {member.get('name', '?')}: {member.get('description', '')}")

        print("\n") # クラスごとに改行


def display_digest(file_path):
    """指定されたJSONファイルを読み込み、内容を要約して表示する"""
    if not os.path.exists(file_path):
        print(f"Error: File not found at '{file_path}'")
        return

    with open(file_path, 'r', encoding='utf-8') as f:
        service_data = json.load(f)

    if not service_data or "service_name" not in service_data:
        print("Error: JSON format is not as expected.")
        return

    print(f"--- Service: {service_data.get('service_name', 'N/A')} ---")
    print(f"URL: {service_data.get('url', 'N/A')}\n")

    for class_info in service_data.get("classes", []):
        class_name = class_info.get('name', 'N/A')
        print(f"Class: {class_name}")
        class_description = class_info.get('description')
        if class_description:
            print(f"  Description: {class_description}")

        # メソッド情報を表示
        if class_info.get("methods"):
            for method in class_info.get("methods", []):
                params_list = []
                for param in method.get("parameters", []):
                    param_type = param.get('type', {}).get('name', 'any')
                    params_list.append(f"{param.get('name', '?')}: {param_type}")
                
                params_str = ", ".join(params_list)
                return_type = method.get('return_type', {}).get('name', 'void')
                method_name = method.get('name', '').split('(')[0]

                print(f"  - {method_name}({params_str}) -> {return_type}")

        # Enumメンバー情報を表示
        if class_info.get("enum_members"):
            print("  Enum Members:")
            for member in class_info.get("enum_members", []):
                print(f"    - {member.get('name', '?')}: {member.get('description', '')}")

        print("\n") # クラスごとに改行

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 digest_viewer.py <path_to_json_file>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    display_digest_only_origin_class(file_path)
    #rlist = get_all_classes(file_path)
    # pprint.pprint(rlist)

