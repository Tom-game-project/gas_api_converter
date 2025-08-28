
import requests
from bs4 import BeautifulSoup
from urllib.parse import urljoin
import time

URL = "https://developers.google.com/apps-script/reference/spreadsheet/auto-fill-series"

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

def inspect_enum_page():
    """EnumページのHTML構造を調べる"""
    soup = get_soup(URL)
    if soup:
        print("\n--- HTML of the Enum page ---")
        # メソッドテーブルに似た構造を探す
        enum_table = soup.select_one("table.members")
        if enum_table:
            print(enum_table.prettify())
        else:
            print("Could not find an enum member table.")

if __name__ == "__main__":
    inspect_enum_page()

