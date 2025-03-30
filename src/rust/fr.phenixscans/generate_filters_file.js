var script = document.createElement('script');
script.src = 'https://code.jquery.com/jquery-3.7.1.min.js';
document.getElementsByTagName('head')[0].appendChild(script);

var filters_json = [
	{
		"type": "title"
	},
	{
		"type": "select",
		"name": "Status",
		"options": []
	},
	{
		"type": "select",
		"name": "Type",
		"options": []
	},
	{
		"type": "group",
		"name": "Tags",
		"filters": []
	},
	{
		"type": "select",
		"name": "Trier par",
		"options": []
	}
]

// Get all genres
for (const genre of $(".multi-select__dropdown .multi-select__option")) {
	filters_json[3].filters.push({
		"type": "genre",
		"name": $("label", genre).text()[0].toUpperCase() + $("label", genre).text().substring(1),
		"id": $("input", genre).attr("value")
	})
}

// Get all status
for (const type of $("option", $(".manga-list__filter-group .manga-list__select")[1])) {
	filters_json[1].options.push($(type).text().trim())
}

// Get all types
for (const type of $("option", $(".manga-list__filter-group .manga-list__select")[0])) {
	filters_json[2].options.push($(type).text().trim())
}

// Get all sort
for (const type of $("option", $(".manga-list__filter-group .manga-list__select")[2])) {
	filters_json[4].options.push($(type).text().trim())
}


console.log(JSON.stringify(filters_json, null, 2));