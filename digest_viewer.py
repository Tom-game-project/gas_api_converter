import json
import sys
import os

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
        print(f"Class: {class_info.get('name', 'N/A')}")
        if class_info.get('description'):
            print(f"  Description: {class_info.get('description')}")

        for method in class_info.get("methods", []):
            params_list = []
            for param in method.get("parameters", []):
                param_type = param.get('type', {}).get('name', 'any')
                params_list.append(f"{param.get('name', '?')}: {param_type}")
            
            params_str = ", ".join(params_list)
            
            return_type = method.get('return_type', {}).get('name', 'void')

            # メソッド名から()や引数部分を削除して整形
            method_name = method.get('name', '').split('(')[0]

            print(f"  - {method_name}({params_str}) -> {return_type}")
        print("\n") # クラスごとに改行

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 digest_viewer.py <path_to_json_file>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    display_digest(file_path)