use aidoku::{
	prelude::*,
	error::Result,
	std::{
		html::Node,
		String, StringRef, Vec, current_date
	},
	Manga, Page, MangaStatus, MangaContentRating, MangaViewer, Chapter
};

//////////////////////////
//// PARSER FUNCTIONS ////
//////////////////////////

pub fn parse_recents(html: Node, result: &mut Vec<Manga>) {
	for page in html.select(".mangalist .manga-item").array() {
		let obj = page.as_node();

		let url = obj.select(".manga-heading a").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);

		let title = obj.select(".manga-heading a").text().read();
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

		result.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url,
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Default
		});
	}
	
}

pub fn parse_recents_popular(html: Node, result: &mut Vec<Manga>) {
	for page in html.select(".hot-thumbnails li").array() {
		let obj = page.as_node();

		let url = obj.select(".manga-name a").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);

		let title = obj.select(".manga-name a").text().read();
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

		result.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url,
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Default
		});
	}
}

pub fn parse_top_manga(html: Node, result: &mut Vec<Manga>) {
	for page in html.select("li").array() {
		let obj = page.as_node();

		let url = obj.select(".media-left a").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);

		let title = obj.select("h5 strong").text().read();
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

		result.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url,
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Default
		});
	}
}

pub fn parse_filterlist(html: Node, result: &mut Vec<Manga>) {
	for page in html.select(".media").array() {
		let obj = page.as_node();

		let url = obj.select(".media-heading .chart-title").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);

		let title = obj.select(".media-heading .chart-title").text().read();
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

		if id.len() > 0 && title.len() > 0 && cover.len() > 0 {
			result.push(Manga {
				id,
				cover,
				title,
				author: String::new(),
				artist: String::new(),
				description: String::new(),
				url,
				categories: Vec::new(),
				status: MangaStatus::Unknown,
				nsfw: MangaContentRating::Safe,
				viewer: MangaViewer::Default
			});
		}
	}
}

pub fn parse_manga(obj: Node, id: String) -> Result<Manga> {
	let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);
	let title = String::from(obj.select(".widget-title").first().text().read().trim());
	let author = String::from(obj.select("a[href*=author]").text().read().trim());
	let artist = String::from(obj.select("a[href*=artist]").text().read().trim());
	let description = String::from(obj.select(".well p").text().read().trim());
	
	let type_str = obj.select(".dl-horizontal dt:contains(\"Type\")").text().read().trim().to_lowercase();
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
		"manhua" => MangaViewer::Scroll,
		_ => MangaViewer::Rtl
	};

	Ok(Manga {
		id,
		cover,
		title,
		author,
		artist,
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

	for chapter in obj.select(".chapters li:not(.volume)").array() {
		let obj = chapter.as_node();

		let title = obj.select(".chapter-title-rtl em").text().read();
		let url = obj.select(".chapter-title-rtl a").attr("href").read();
		let id = String::from(&url.replace("https://lelscanvf.com/manga/", ""));

		let split_url :Vec<&str>= url.split("/").collect();
		let chapter = split_url[split_url.len() - 1].parse().unwrap();

		let mut date_updated = StringRef::from(&obj.select(".date-chapter-title-rtl").text().read().trim())
			.0
			.as_date("d MMM yyyy", Some("fr"), None)
			.unwrap_or(-1.0);

		if date_updated == -1.0 {
			date_updated = current_date();
		}

		chapters.push(Chapter{
			id,
			title,
			volume: -1.0,
			chapter,
			date_updated,
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

		let text = String::from(obj.attr("alt").read().trim());

		pages.push(Page {
			index: i as i32,
			url,
			base64: String::new(),
			text,
		});
		i += 1;
	}
	Ok(pages)
}

pub fn is_last_page(obj: Node) -> bool {
	return obj.select(".pagination li").last().has_class("disabled")
}

