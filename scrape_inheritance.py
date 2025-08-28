import requests
from bs4 import BeautifulSoup
import time

URL = "https://developers.google.com/apps-script/reference/document/paragraph"

def get_soup(url):
    print(f"INFO: Accessing {url}")
    try:
        time.sleep(1)
        response = requests.get(url)
        response.raise_for_status()
        return BeautifulSoup(response.content, "html.parser")
    except requests.exceptions.RequestException as e:
        print(f"ERROR: Failed to fetch {url}: {e}")
        return None

def inspect_inheritance_page():
    """クラスページの継承情報のHTML構造を調べる"""
    soup = get_soup(URL)
    if soup:
        print("\n--- HTML of the class page ---")
        print(soup.prettify())

if __name__ == "__main__":
    inspect_inheritance_page()