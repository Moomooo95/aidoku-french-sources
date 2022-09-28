use aidoku::{
	prelude::*, error::Result, std::String, std::Vec, std::html::Node,
	Filter, FilterType, Manga, Page, MangaStatus, MangaContentRating, MangaViewer, Chapter,
};

pub fn parse_recents(html: Node, result: &mut Vec<Manga>) {
	for page in html.select(".mangalist .manga-item").array() {
		let obj = page.as_node();

		let url = obj.select(".manga-heading a").attr("href").read();
		let mut id = String::new();
		let mut i = 0;
		for s in url.split('/') {
			if i == 4 {
				id = String::from(s);
				break;
			}
			i+=1;
		}
		let title = obj.select(".manga-heading a").text().read();
		let img = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

		result.push(Manga {
			id,
			cover: img,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url: String::new(),
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Default
		});
	}
}

pub fn parse_search(html: Node, result: &mut Vec<Manga>) {
	for page in html.select(".media").array() {
		let obj = page.as_node();

		let id = obj.select(".media-heading .chart-title").attr("href").read();
		let title = obj.select(".media-heading .chart-title").text().read();
		let img = format!("https:{}", obj.select(".media-left .thumbnail img").attr("src").read());

		if id.len() > 0 && title.len() > 0 && img.len() > 0 {
			result.push(Manga {
				id,
				cover: img,
				title,
				author: String::new(),
				artist: String::new(),
				description: String::new(),
				url: String::new(),
				categories: Vec::new(),
				status: MangaStatus::Unknown,
				nsfw: MangaContentRating::Safe,
				viewer: MangaViewer::Default
			});
		}
	}
}

pub fn parse_filterlist(html: Node, result: &mut Vec<Manga>) {
	for page in html.select(".media").array() {
		let obj = page.as_node();

		let id = obj.select(".media-heading .chart-title").attr("href").read();
		let title = obj.select(".media-heading .chart-title").text().read();
		let img = format!("https:{}", obj.select(".media-left .thumbnail img").attr("src").read());

		if id.len() > 0 && title.len() > 0 && img.len() > 0 {
			result.push(Manga {
				id,
				cover: img,
				title,
				author: String::new(),
				artist: String::new(),
				description: String::new(),
				url: String::new(),
				categories: Vec::new(),
				status: MangaStatus::Unknown,
				nsfw: MangaContentRating::Safe,
				viewer: MangaViewer::Default
			});
		}
	}
}

pub fn parse_manga(obj: Node, id: String) -> Result<Manga> {
	let title = obj.select(".lazy").attr("alt").read();
	let cover = format!("https:{}", obj.select(".img-responsive").attr("src").read());
	let description = obj.select(".well p").text().read();
	let type_str = obj.select(".dl-horizontal > dd:nth-child(8) > a:nth-child(1)").text().read().to_lowercase();
	let status_str = obj.select(".label").text().read().trim().to_lowercase();

	let url = format!("https://lelscanvf.com/manga/{}", &id);

	let mut categories: Vec<String> = Vec::new();
	obj.select("a[href*=genre]")
		.array()
		.for_each(|tag| categories.push(tag.as_node().text().read()));

	let status = if status_str.contains("ongoing") {
		MangaStatus::Ongoing
	} else if status_str.contains("finished") {
		MangaStatus::Completed
	} else {
		MangaStatus::Unknown
	};

	let nsfw = if obj.select(".alert-warning").text().read().contains("Mature") {
		MangaContentRating::Nsfw
	} else if categories.contains(&String::from("Ecchi")) {
		MangaContentRating::Suggestive
	} else {
		MangaContentRating::Safe
	};

	let viewer = match type_str.as_str() {
		"manga" => MangaViewer::Rtl,
		"manhwa" => MangaViewer::Scroll,
		_ => MangaViewer::Rtl
	};

	Ok(Manga {
		id,
		cover,
		title,
		author: String::new(),
		artist: String::new(),
		description,
		url,
		categories,
		status,
		nsfw,
		viewer
	})
}

pub fn get_chaper_list(obj: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for chapter in obj.select(".chapters li").array() {
		let obj = chapter.as_node();
		let title = obj.select("h5 em").text().read();
		let url = obj.select("h5 a").attr("href").read();
		let id = String::from(&url[28..]);


		let split = url.as_str().split("/");
		let vec = split.collect::<Vec<&str>>();
		let chap_num = vec[vec.len() - 1].parse().unwrap();

		chapters.push(Chapter{
			id,
			title,
			volume: -1.0,
			chapter: chap_num,
			date_updated: -1.0,
			scanlator: String::new(),
			url,
			lang: String::from("fr"),
		});
	}
	Ok(chapters)
}

pub fn get_page_list(obj: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let mut i = 0;
	for page in obj.select("#all img").array() {
		let obj = page.as_node();
		let mut url = String::from(obj.attr("data-src").read().trim());

		if !url.contains("https:") {
			url = format!("https:{}", obj.attr("data-src").read().trim());
		}

		pages.push(Page {
			index: i as i32,
			url,
			base64: String::new(),
			text: String::new(),
		});
		i += 1;
	}
	Ok(pages)
}

pub fn get_filtered_url(filters: Vec<Filter>, page: i32, url: &mut String) {
	let mut is_searching = false;
	let mut query = String::new();
	// let mut search_string = String::new();
	url.push_str("https://lelscanvf.com");

	for filter in filters {
		match filter.kind {
			// FilterType::Title => {
			// 	if let Ok(filter_value) = filter.value.as_string() {
			// 		// filter_value.read().to_lowercase();
			// 		search_string.push_str(urlencode(filter_value.read().to_lowercase()).as_str());
			// 		is_searching = true;
			// 	}
			// },
			FilterType::Genre => {
				query.push_str("&cat=");
				query.push_str(&urlencode(filter.name.as_str().to_lowercase()));
				is_searching = true;
			},
			FilterType::Select => {
				if filter.name.as_str() == "Sort By" {
					query.push_str("&sortBy=");
					match filter.value.as_int().unwrap_or(-1) {
						0 =>  query.push_str("name"),
						1 =>  query.push_str("rate"),
						_ => continue,
					}
					if filter.value.as_int().unwrap_or(-1) > 0 {
						is_searching = true;
					}
				}
				if filter.name.as_str() == "Sort" {
					query.push_str("&asc=");
					match filter.value.as_int().unwrap_or(-1) {
						0 =>  query.push_str("true"),
						1 =>  query.push_str("false"),
						_ => continue,
					}
					if filter.value.as_int().unwrap_or(-1) > 0 {
						is_searching = true;
					}
				}
			},
			_ => continue,
		}
	}

	if is_searching {
		url.push_str("/filterList?");
		url.push_str("page=");
		url.push_str(&i32_to_string(page));
		url.push_str(&query);
		url.push_str("&alpha=&author=&artist=&tag=");
	}
}

pub fn parse_incoming_url(url: String) -> String {
    // https://mangapill.com/manga/6290/one-piece-pirate-recipes
    // https://mangapill.com/chapters/6290-10006000/one-piece-pirate-recipes-chapter-6

    let split = url.as_str().split("/");
    let vec = split.collect::<Vec<&str>>();
    let mut manga_id = String::from("/manga/");

    if url.contains("/chapters/") {
        let split  = vec[vec.len() - 2].split("-");
        let ch_vec = split.collect::<Vec<&str>>();
        manga_id.push_str(ch_vec[0]);
    } else {
        manga_id.push_str(vec[vec.len() - 2]);
    }
    manga_id.push_str("/");
    manga_id.push_str(vec[vec.len() - 1]);
    return manga_id;
}

// HELPER FUNCTIONS

pub fn i32_to_string(mut integer: i32) -> String {
	if integer == 0 {
		return String::from("0");
	}
	let mut string = String::with_capacity(11);
	let pos = if integer < 0 {
		string.insert(0, '-');
		1
	} else {
		0
	};
	while integer != 0 {
		let mut digit = integer % 10;
		if pos == 1 {
			digit *= -1;
		}
		string.insert(pos, char::from_u32((digit as u32) + ('0' as u32)).unwrap());
		integer /= 10;
	}
	return string;
}

pub fn urlencode(string: String) -> String {
	let mut result: Vec<u8> = Vec::with_capacity(string.len() * 3);
	let hex = "0123456789abcdef".as_bytes();
	let bytes = string.as_bytes();

	for byte in bytes {
		let curr = *byte;
		if (b'a' <= curr && curr <= b'z')
			|| (b'A' <= curr && curr <= b'Z')
			|| (b'0' <= curr && curr <= b'9') {
				result.push(curr);
		} else {
			result.push(b'%');
			result.push(hex[curr as usize >> 4]);
			result.push(hex[curr as usize & 15]);
		}
	}

	String::from_utf8(result).unwrap_or(String::new())
}