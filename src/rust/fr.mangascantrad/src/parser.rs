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

// parse manga with basic details
pub fn parse_catalogue(html: Node, mangas: &mut Vec<Manga>) {
	for page in html.select(".tab-content-wrap .c-tabs-item__content").array() {
		let obj = page.as_node();

		let url = obj.select(".tab-thumb a").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);
		let cover = obj.select(".tab-thumb.c-image-hover a img").attr("data-src").read().replace("-193x278", "");
		let title = obj.select(".post-title h3").text().read();

		println!("{}", cover);

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
pub fn is_last_pages_catalogue(html: Node) -> bool {
	let is_last = String::from(html.select(".wp-pagenavi .last").text().read().trim()).len() as i64;
	return is_last == 0;
}

// parse mangas with full details
pub fn parse_manga_details(manga_obj: Node, id: String) -> Result<Manga> {	
	let cover = manga_obj.select(".site-content .summary_image img").attr("data-src").read();
	let title = manga_obj.select(".site-content .post-title h1").text().read();
	let author = manga_obj.select("a[href*=author]").text().read();
	let artist = manga_obj.select("a[href*=artist]").text().read();
	let description = manga_obj.select(".description-summary p").text().read();
	let url = format!("https://manga-scantrad.net/manga/{}", &id);

	let mut categories: Vec<String> = Vec::new();
	manga_obj.select(".genres-content a[href*=manga-genre]")
		.array()
		.for_each(|tag| categories.push(tag.as_node().text().read()));

	let status_str = manga_obj.select("div.post-content_item:nth-child(2) > div:nth-child(2)").text().read().trim().to_lowercase();
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

	let type_str = manga_obj.select("div.post-content_item:nth-child(9) > div:nth-child(2)").text().read().to_lowercase();

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
pub fn parse_chapter_list(chapter_obj: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();
	for chapter in chapter_obj.select("ul .wp-manga-chapter").array() {
		let chapter_obj = chapter.as_node();

		let url = chapter_obj.select("a").attr("href").read();
		let id = String::from(&url.replace("https://manga-scantrad.net/manga/", ""));

		let mut volume = -1.0;
		let mut chapter = -1.0;
		let mut title = String::new();

		let chap_title_str = String::from(chapter_obj.select("a").text().read().trim());
		if chap_title_str.contains("-") {
			let split_title :Vec<&str>= chap_title_str.split("-").collect();

			if split_title[0].contains("Vol") {
				let split_title :Vec<&str>= split_title[0].split(" ").collect(); // ["Vol.1", "Ch.30"]

				let split_vol :Vec<&str>= split_title[0].split(".").collect();
				volume = String::from(split_vol[1]).parse().unwrap();

				let split_chp :Vec<&str>= split_title[1].split(".").collect();
				chapter = String::from(split_chp[1]).parse().unwrap();

			} else if split_title[0].contains("Chapitre") {
				let split_chp :Vec<&str>= split_title[0].split(" ").collect();
				chapter = String::from(split_chp[1]).parse().unwrap();
			}
			
			title = String::from(split_title[1].trim());
		}
		else {

			if !chap_title_str.contains("Vol") && !chap_title_str.contains("Chapitre") {
				let mut split_title :Vec<&str>= chap_title_str.trim().split(" ").collect();

				if split_title.len() > 1 {
					chapter = String::from(split_title[0].trim()).parse().unwrap();
					split_title.remove(0);
					title = String::from(split_title.join(" "));
				}
				else {
					chapter = String::from(chap_title_str.trim()).parse().unwrap();
				}
			}
			else {
				let split_str :Vec<&str>= id.split("/").collect();
				let chapter_str = String::from(split_str[1]);
				let split_chapter_str :Vec<&str>= chapter_str.split("-").collect();
				chapter = String::from(split_chapter_str[split_chapter_str.len() - 1]).parse().unwrap();
			}
		}
		
		let mut date_updated = StringRef::from(&chapter_obj.select(".chapter-release-date").text().read().trim())
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
pub fn parse_chapter_details(chapter_details_obj: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let mut i = 0;
	for page in chapter_details_obj.select(".entry-content .reading-content .page-break img").array() {
		let url = String::from(page.as_node().attr("data-src").read().trim());

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
