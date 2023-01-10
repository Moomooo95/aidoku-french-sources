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
pub fn parse_catalogue(html: Node) -> Vec<Manga> {
	let mut mangas: Vec<Manga> = Vec::new();

	for manga in html.select(".tab-content-wrap .c-tabs-item__content").array() {
		let obj = manga.as_node().expect("html array not an array of nodes");

		// check if is not anime
		if obj.select(".tab-meta .latest-chap .chapter a").attr("href").read().contains("episode") { continue; }

		let cover = get_url_image(obj.select(".tab-thumb.c-image-hover a img"));
		let title = obj.select(".post-title h3").text().read();
		let url = obj.select(".tab-thumb a").attr("href").read();
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

// check if is last page of catalogue
pub fn is_not_last_pages_catalogue(html: Node) -> bool {
	return html.select(".wp-pagenavi .nextpostslink").text().read().trim().len() != 0;
}

// parse mangas with full details
pub fn parse_manga_details(html: Node, id: String) -> Result<Manga> {	
	let cover = get_url_image(html.select(".site-content .summary_image img"));
	let title = html.select(".site-content .post-title h1").text().read();
	let author = html.select("a[href*=author]").text().read();
	let artist = html.select("a[href*=artist]").text().read();
	let description = html.select(".description-summary p").text().read();
	let url = format!("https://manga-scantrad.io/manga/{}", &id);

	let mut categories: Vec<String> = Vec::new();
	html.select(".genres-content a[href*=manga-genre]")
		.array()
		.for_each(|tag| categories.push(tag.as_node().expect("html array not an array of nodes").text().read()));

	let status_str = html.select("div.post-content_item:nth-child(2) > div:nth-child(2)").text().read().trim().to_lowercase();
	let status = if status_str.contains("en cours") {
		MangaStatus::Ongoing
	} else if status_str.contains("terminé") {
		MangaStatus::Completed
	} else if status_str.contains("en pause") {
		MangaStatus::Hiatus
	} else if status_str.contains("annulé") {
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

	let type_str = html.select("div.post-content_item:nth-child(9) > div:nth-child(2)").text().read().to_lowercase();

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

// parse all chapter of manga
pub fn parse_chapter_list(html: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for chapter in html.select("ul .wp-manga-chapter").array() {
		let obj = chapter.as_node().expect("html array not an array of nodes");

		let url = String::from(obj.select("a").attr("href").read().trim());
		let id = String::from(url.split("/").enumerate().nth(5).expect("id").1.trim());

		// get title of chapter if exist
		let mut title = String::new();
		let chapter_info = String::from(obj.select("a").text().read().trim());
		if chapter_info.contains("-") {
			title = String::from(chapter_info.split("-").enumerate().nth(1).expect("title").1.trim().replace("_", " "));
		}

		// get volume and chapter number
		let details_chapter = String::from(url.split("/").enumerate().nth(5).expect("details").1.trim());
		let details_chapter_array: Vec<&str> = details_chapter.split("-").collect();
		
		// get volume number if exist
		let mut volume = -1.0;
		if details_chapter.contains("vol-") {
			volume = details_chapter_array[details_chapter_array.iter().position(|&x| x.contains("vol")).unwrap() + 1].parse().unwrap()
		}

		// get chapter number
		let mut chapter = -1.0;
		if details_chapter.contains("chapter-") || details_chapter.contains("chapitre-") || details_chapter.contains("ch-") || details_chapter.contains("chap-") {
			let index_chapter = details_chapter_array.iter().position(|&x| x.contains("chapter") || x.contains("chapitre") || x.contains("ch") || x.contains("chap")).unwrap() + 1;
			let mut chapter_str = String::from(details_chapter_array[index_chapter].trim());

			if chapter_str.contains("_") {
				chapter_str = String::from(chapter_str.split("_").enumerate().nth(0).expect("chapter").1.trim());
			}

			if details_chapter_array.len() > index_chapter + 2  && details_chapter_array[index_chapter + 1].trim().parse().unwrap_or(-1.0) != -1.0{
				chapter_str = format!("{}.{}", chapter_str, details_chapter_array[index_chapter + 1].trim());
			}
			
			chapter = chapter_str.parse().unwrap();
		}

		// get release date of chapter
		let mut date_updated = StringRef::from(&obj.select(".chapter-release-date").text().read().trim())
			.0
			.as_date("d MMMM yyyy", Some("fr"), None)
			.unwrap_or(-1.0);

		if date_updated == -1.0 {
			date_updated = current_date();
		}

		chapters.push(Chapter{
			id,
			title,
			volume,
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
pub fn parse_chapter_details(html: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let mut i = 0;
	for image_obj in html.select(".entry-content .reading-content .page-break img").array() {
		let url = get_url_image(image_obj.as_node()?);

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
