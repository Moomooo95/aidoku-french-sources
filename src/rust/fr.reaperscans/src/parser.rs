use aidoku::{
	prelude::*,
	error::Result,
	std::{
		html::Node,
		String, StringRef, Vec, current_date
	},
	Manga, Page, MangaStatus, MangaContentRating, MangaViewer, Chapter
};
use crate::helper::get_url_image;


//////////////////////////
//// PARSER FUNCTIONS ////
//////////////////////////

// parse manga with basic details
pub fn parse_serie(html: Node) -> Vec<Manga> {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select(".row.c-tabs-item__content").array() {
		let obj = page.as_node().expect("html array not an array of nodes");

		let cover = get_url_image(&obj);
		let title = obj.select("h3 a").text().read();
		let url = obj.select("h3 a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(4).expect("id").1.trim());

		mangas.push(Manga {
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
			viewer: MangaViewer::Rtl
		});
	}

	return mangas;
}

// check if is last page of serie
pub fn is_not_last_pages_serie(html: Node) -> bool {
	return html.select("nav.navigation-ajax").text().read().trim().len() != 0;
}

// parse mangas with full details
pub fn parse_manga_details(manga_obj: Node, id: String) -> Result<Manga> {	
	let cover = get_url_image(&manga_obj);
	let title = manga_obj.select(".container .post-title h1").text().read();
	let author = manga_obj.select("a[href*=author]").text().read();
	let artist = manga_obj.select("a[href*=artist]").text().read();
	let description = manga_obj.select("#nav-profile p").text().read();
	let url = format!("https://reaperscans.fr/serie/{}", &id);

	let mut categories: Vec<String> = Vec::new();
	manga_obj.select("a[href*=genre][rel=tag]")
		.array()
		.for_each(|tag| categories.push(tag.as_node().expect("html array not an array of nodes").text().read()));

	let status_str = manga_obj.select("div.post-content_item:nth-child(2) > .summary-content").text().read().trim().to_lowercase();
	let status = if status_str.contains("ongoing") {
		MangaStatus::Ongoing
	} else if status_str.contains("end") {
		MangaStatus::Completed
	} else if status_str.contains("canceled") || status_str.contains("dropped") {
		MangaStatus::Cancelled
	} else if status_str.contains("on hold") {
		MangaStatus::Hiatus
	} else {
		MangaStatus::Unknown
	};

	let type_str_1 = manga_obj.select("div.post-content_item:nth-child(9) > .summary-content").text().read().to_lowercase();
	let type_str_2 :Vec<&str>= type_str_1.split(",").collect();
	let type_str  = String::from(type_str_2[0].trim());
	let nsfw = MangaContentRating::Safe;
	let viewer = match type_str.as_str() {
		"manga" => MangaViewer::Rtl,
		"manhua" => MangaViewer::Scroll,
		"manhwa" => MangaViewer::Scroll,
		"webtoon" => MangaViewer::Scroll,
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

// parse all chapter of manga
pub fn parse_chapter_list(chapter_obj: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for chapter in chapter_obj.select(".listing-chapters_wrap .wp-manga-chapter").array() {
		let chapter_obj = chapter.as_node().expect("html array not an array of nodes");

		let url = chapter_obj.select("a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(5).expect("id").1.trim());

		let chap_title_str = String::from(chapter_obj.select(".chapter-manhwa-title").text().read().trim());
		let mut title = String::new();
		if chap_title_str.contains("-") {
			let split_title :Vec<&str>= chap_title_str.split("-").collect();
			title = String::from(split_title[1].trim());
		}

		let split_str :Vec<&str>= chap_title_str.split(" ").collect();
		let chapter = String::from(split_str[1]).parse().unwrap();

		let mut date_updated = StringRef::from(&chapter_obj.select(".chapter-release-date i").text().read().trim())
			.0
			.as_date("dd/MM/yyyy", Some("fr"), None)
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

// parse all images of chapters
pub fn parse_chapter_details(chapter_details_obj: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let mut i = 0;
	for page in chapter_details_obj.select(".entry-content .reading-content .page-break").array() {
		let url = get_url_image(&page.as_node()?);

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
