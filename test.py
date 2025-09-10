# https://developers.google.com/apps-script/reference/document/element?hl=ja

import requests
from bs4 import BeautifulSoup

def get_table():
    a = requests.get("https://developers.google.com/apps-script/reference/document/element")
    soup = BeautifulSoup(a.text, "html.parser")
    
    target = soup.find("table", class_="member type")
    return target
    

if __name__ == "__main__":
    class_table = get_table()
    print(class_table)
