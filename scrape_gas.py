import requests
from bs4 import BeautifulSoup
import json
from urllib.parse import urljoin
import re
import time
import os

BASE_URL = "https://developers.google.com/apps-script/reference"

def get_soup(url):
    """指定されたURLからBeautifulSoupオブジェクトを取得する（1秒待機付き）"""
    print(f"INFO: Accessing {url}")
    try:
        time.sleep(1) # サーバーへの配慮
        response = requests.get(url)
        response.raise_for_status()
        return BeautifulSoup(response.content, "html.parser")
    except requests.exceptions.RequestException as e:
        print(f"ERROR: Failed to fetch {url}: {e}")
        return None

def extract_type_info(element):
    """要素から型情報を抽出する。リンクがあればURLも取得する。"""
    if not element:
        return {"name": "void", "url": None}
    
    code_tag = element.find("code")
    if not code_tag:
        return {"name": element.get_text(strip=True), "url": None}

    link = code_tag.find("a")
    if link and link.get("href"):
        type_name = ''.join(link.find_all(string=True, recursive=False)).strip()
        type_url = urljoin(BASE_URL, link["href"])
        return {"name": type_name, "url": type_url}
    else:
        type_name = code_tag.get_text(strip=True)
        return {"name": type_name, "url": None}

def parse_method_doc(doc_div):
    """単一のメソッドドキュメントブロックを解析する"""
    method_info = {"parameters": []}

    signature_tag = doc_div.find("h3").find("code")
    method_info["name"] = signature_tag.get_text(strip=True) if signature_tag else ""

    description_tag = doc_div.find("p")
    method_info["description"] = description_tag.get_text(strip=True) if description_tag else ""

    params_heading = doc_div.find("h4", id=re.compile(r'^parameters'))
    if params_heading:
        params_table = params_heading.find_next_sibling("table", class_="function param")
        if params_table:
            for row in params_table.find_all("tr")[1:]:
                cols = row.find_all("td")
                if len(cols) == 3:
                    param_name = cols[0].get_text(strip=True)
                    param_type_info = extract_type_info(cols[1])
                    param_desc = cols[2].get_text(strip=True)
                    method_info["parameters"].append({
                        "name": param_name,
                        "type": param_type_info,
                        "description": param_desc
                    })
    
    return_heading = doc_div.find("h4", id=re.compile(r'^return'))
    if return_heading:
        return_p = return_heading.find_next_sibling("p")
        method_info["return_type"] = extract_type_info(return_p)
    else:
        method_info["return_type"] = {"name": "void", "url": None}

    return method_info

def parse_class_page(class_url):
    """クラスのページを解析し、メソッドやプロパティ、継承情報を抽出する"""
    print(f"  Scraping class: {class_url}")
    soup = get_soup(class_url)
    if not soup:
        return None

    class_name_tag = soup.find("h1", class_="devsite-page-title")
    class_name = ""
    if class_name_tag:
        # Extract text only from the main tag, ignoring child tags like spans
        class_name = class_name_tag.find(string=True, recursive=False).strip()

    description_tag = soup.select_one("div[itemprop='articleBody'] > .type.doc > p")
    description = description_tag.get_text(strip=True) if description_tag else ""

    class_info = {
        "name": class_name,
        "url": class_url,
        "description": description,
        "methods": [],
        "enum_members": [],
        "implementing_classes": [] # インターフェースを実装するクラスのリスト
    }

    # "Implemented by:" (インターフェースの場合) - ユーザー提供のロジックを使用
    if "Interface" in class_name:
        class_table = soup.find("table", class_="member type")
        if class_table:
            class_names = []
            # `<a>` タグを持つ `<code>` タグを探す
            for link in class_table.select('td > code > a'):
                class_names.append(link.get_text(strip=True))
            class_info["implementing_classes"] = class_names

    # Enum型の場合、メンバーを抽出
    if class_name.startswith("Enum"):
        enum_table = soup.select_one("table.members.property")
        if enum_table:
            for row in enum_table.find_all("tr")[1:]:
                cols = row.find_all("td")
                if len(cols) == 3:
                    member_name = cols[0].get_text(strip=True)
                    member_desc = cols[2].get_text(strip=True)
                    class_info["enum_members"].append({
                        "name": member_name,
                        "description": member_desc
                    })
    else:
        # Enumでない場合のみメソッドを抽出
        method_docs = soup.select("div.function.doc[id]")
        for doc_div in method_docs:
            method_details = parse_method_doc(doc_div)
            class_info["methods"].append(method_details)

    return class_info

def scrape_gas_docs():
    """Google Apps Scriptのリファレンスドキュメント全体をスクレイピングする"""
    print("Starting full scraping process...")
    top_soup = get_soup(f"{BASE_URL}/")
    if not top_soup:
        print("Failed to fetch the main reference page. Aborting.")
        return

    service_pattern = re.compile(r'^/apps-script/reference/[^/]+/?$')
    service_links = top_soup.select('a.devsite-nav-title[href*="/apps-script/reference/"]')
    
    service_main_urls = set()
    for link in service_links:
        href = link.get("href", "")
        if service_pattern.match(href):
            full_url = urljoin(BASE_URL, href)
            service_main_urls.add(full_url)

    print(f"Found {len(service_main_urls)} unique services to scrape.")

    if not service_main_urls:
        print("ERROR: No service URLs found. Exiting.")
        return

    for service_url in sorted(list(service_main_urls)):
        service_name = service_url.strip("/").split("/")[-1]
        print(f"\n--- Scraping service: {service_name} ---")
        
        service_soup = get_soup(service_url)
        if not service_soup:
            continue

        service_data = {"service_name": service_name, "url": service_url, "classes": []}

        class_links = service_soup.select('div.devsite-article-body td a[href*="/apps-script/reference/"]')
        print(f"Found {len(class_links)} potential class links in {service_name}.")

        class_urls = set()
        for link in class_links:
            href = link.get("href", "")
            href_without_anchor = href.split('#')[0]
            class_url = urljoin(service_url, href_without_anchor)
            
            if class_url.startswith(service_url) and class_url != service_url:
                class_urls.add(class_url)

        for class_url in sorted(list(class_urls)):
            class_info = parse_class_page(class_url)
            if class_info:
                service_data["classes"].append(class_info)

        output_filename = f"api-def2/{service_name}.json"
        with open(output_filename, "w", encoding="utf-8") as f:
            json.dump(service_data, f, ensure_ascii=False, indent=2)
        print(f"Service '{service_name}' data saved to {output_filename}")

    print("\n--- Full scraping finished. ---")

if __name__ == "__main__":
    scrape_gas_docs()