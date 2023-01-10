
genres = []

for (genre of $('#search-advanced .checkbox-group .checkbox').toArray()) {
  genres.push({
    "type": "genre",
    "name": $(genre).children('label').text().trim(),
    "id": $(genre).children('input').attr('value').trim(),
    "canExclude": false
  })
}

console.log(genres)