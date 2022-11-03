use aidoku::{
	prelude::*,
	error::Result,
	std::{
		html::Node,
		String, StringRef, Vec, current_date
	},
	Manga, Page, MangaStatus, MangaContentRating, MangaViewer, Chapter
};
use core::str::FromStr;
use crate::get_url_image;

//////////////////////////
//// PARSER FUNCTIONS ////
//////////////////////////

// parse manga with basic details
pub fn parse_catalogue(html: Node, mangas: &mut Vec<Manga>) {
	for page in html.select(".row .c-tabs-item__content").array() {
		let obj = page.as_node();

		let url = obj.select("h3 a").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);
		let cover = get_url_image(&obj);
		let title = obj.select("h3 a").text().read();

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
			viewer: MangaViewer::Default
		});
	}
}

// parse total pages of catalogue
pub fn parse_total_pages_catalogue(html: Node) -> i32 {
	let nb_page = String::from(html.select(".search-wrap h1.h4").text().read().trim());
	let split_pages :Vec<&str>= nb_page.split(" ").collect();
	return (i32::from_str(split_pages[0]).unwrap()).div_ceil(12);
}

// parse mangas with full details
pub fn parse_manga_details(manga_obj: Node, id: String) -> Result<Manga> {	
	let cover = get_url_image(&manga_obj);
	let title = manga_obj.select(".container .post-title h1").text().read();
	let author = manga_obj.select("a[href*=auteur]").text().read();
	let artist = manga_obj.select("a[href*=artist]").text().read();
	let description = manga_obj.select(".container .tab-summary .manga-excerpt").text().read();
	let url = format!("https://mangas-origines.fr/catalogues/{}", &id);

	let mut categories: Vec<String> = Vec::new();
	manga_obj.select("a[href*=genre][rel=tag]")
		.array()
		.for_each(|tag| categories.push(tag.as_node().text().read()));

	let status_str = manga_obj.select("div.post-content_item:nth-child(2) > div:nth-child(2)").text().read().trim().to_lowercase();
	let status = if status_str.contains("en cours") {
		MangaStatus::Ongoing
	} else if status_str.contains("terminé") {
		MangaStatus::Completed
	} else if status_str.contains("en pause") {
		MangaStatus::Hiatus
	} else if status_str.contains("abandonné") {
		MangaStatus::Cancelled
	} else {
		MangaStatus::Unknown
	};

	let nsfw = if categories.contains(&String::from("Hentai")) || categories.contains(&String::from("Adulte")) || categories.contains(&String::from("Sexe")) {
		MangaContentRating::Nsfw
	} else if categories.contains(&String::from("Ecchi")) {
		MangaContentRating::Suggestive
	} else {
		MangaContentRating::Safe
	};

	let type_str_1 = manga_obj.select("div.post-content_item:nth-child(9) > div:nth-child(2)").text().read().to_lowercase();
	let type_str_2 :Vec<&str>= type_str_1.split(",").collect();
	let type_str  = String::from(type_str_2[0].trim());
	let viewer = match type_str.as_str() {
		"manga" => MangaViewer::Rtl,
		"manhua" => MangaViewer::Scroll,
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
	for chapter in chapter_obj.select(".wp-manga-chapter").array() {
		let chapter_obj = chapter.as_node();

		let url = chapter_obj.select("a").attr("href").read();
		let id = String::from(&url.replace("https://mangas-origines.fr/manga/", ""));

		let chap_title_str = String::from(chapter_obj.select("a").text().read().trim());
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
	for page in chapter_details_obj.select(".container .reading-content div").array() {
		let url = get_url_image(&page.as_node());
		// println!("{}", String::from(chapter_details_obj.html().read()));

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
