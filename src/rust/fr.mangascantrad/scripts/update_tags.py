from bs4 import BeautifulSoup
import requests
import json
import os

response = requests.get('https://manga-scantrad.net/?s=&post_type=wp-manga', headers={"Host": "manga-scantrad.net"})
soup = BeautifulSoup(response.content, features='html.parser')

print(response.content)
genres = []
for genre in soup.select('#search-advanced .checkbox-group .checkbox'):
    genres.append({
        "type": "genre",
        "name": genre.find('label').text,
        "id": genre.find('input').get('value'),
        "canExclude": False
    })

filters_json = os.path.join(
    os.path.dirname(os.path.realpath(__file__)), "..", "res", "filters.json"
)
file = []
with open(filters_json, "r") as f:
    file = json.load(f)
    
file[6]['filters'] = genres

with open(filters_json, "w") as f:
    json.dump(file, f, indent="\t")
    f.write("\n")