from bs4 import BeautifulSoup
import requests
import json
import os

response = requests.get('https://lelscanvf.com/manga-list', headers={"Host": "www.lelscanvf.com"})
soup = BeautifulSoup(response.content, features='html.parser')

genres = []
for genre in soup.select('.list-category li a'):
    genres.append({
        "type": "genre",
        "name": genre.text,
        "id": genre.get('href').split('/').pop().split('=').pop(),
        "canExclude": False
    })

filters_json = os.path.join(
    os.path.dirname(os.path.realpath(__file__)), "..", "res", "filters.json"
)
file = []
with open(filters_json, "r") as f:
    file = json.load(f)
    
file[3]['filters'] = genres

with open(filters_json, "w") as f:
    json.dump(file, f, indent="\t")
    f.write("\n")