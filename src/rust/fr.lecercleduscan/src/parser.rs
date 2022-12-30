use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		html::Node,
		String, StringRef, Vec, current_date, json
	},
	Manga, Page, MangaStatus, MangaContentRating, MangaViewer, Chapter
};

//////////////////////////
//// PARSER FUNCTIONS ////
//////////////////////////

// parse manga with basic details
pub fn parse_mangas(html: Node, need_cover_request: bool) -> Vec<Manga>  {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select(".group").array() {
		let obj = page.as_node().expect("html array not an array of nodes");
		
		let title = obj.select(">.title a").attr("title").read();
		let url = obj.select(">.title a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(4).expect("id").1.trim());

		let cover = if need_cover_request {
			let url = format!("https://lel.lecercleduscan.com/series/{}", &id);
			let html = Request::new(&url, HttpMethod::Get).html();
			String::from(html.expect("page manga details").select(".thumbnail img").attr("src").read().trim())
		} else {
			String::from(obj.select(".preview").attr("src").read().trim())
		};
		

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

// check if is last page of list manga
pub fn check_not_last_page(html: Node) -> bool {
	return html.select(".next").text().read().len() != 0;
}

// parse mangas with full details
pub fn parse_manga_details(manga_obj: Node, id: String) -> Result<Manga> {	
	let cover = String::from(manga_obj.select(".thumbnail img").attr("src").read().trim());
	let title = manga_obj.select(".large.comic .title").text().read();

	let mut author = String::new();
	let mut artist = String::new();
	let mut description = String::new();
	for item in manga_obj.select(".large.comic .info").html().read().split("<br>") {
		let split :Vec<&str> = item.trim().split(":").collect();

		match split[0].trim() {
			"<b>Author</b>" => author = String::from(split[1].trim()),
			"<b>Artist</b>" => artist = String::from(split[1].trim()),
			"<b>Synopsis</b>" => description = String::from(split[1].trim()),
			_ => ()
		}
	}
	let url = format!("https://lel.lecercleduscan.com/series/{}", &id);

	Ok(Manga {
		id,
		cover,
		title,
		author,
		artist,
		description,
		url,
		categories: Vec::new(),
		status: MangaStatus::Unknown,
		nsfw: MangaContentRating::Safe,
		viewer: MangaViewer::Scroll
	})
}

// parse all chapter of manga
pub fn parse_chapter_list(chapter_obj: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for chapter in chapter_obj.select(".list .element").array() {
		let chapter_obj = chapter.as_node().expect("html array not an array of nodes");

		let url = chapter_obj.select(".title a").attr("href").read();
		let id = String::from(&url.replace("https://lel.lecercleduscan.com/read/", ""));

		let split_url :Vec<&str>= url.split("/").collect();
		let volume = if split_url[6] == "0" {
			-1.0
		} else {
			String::from(split_url[6]).parse().unwrap()
		} as f32;

		let chapter = if split_url[8] == "" {
			String::from(split_url[7]).parse().unwrap()
		} else {
			String::from(format!("{}.{}", split_url[7], split_url[8])).parse().unwrap()
		};

		let chap_title_str = chapter_obj.select(".title a").text().read();
		let mut title = String::new();
		if chap_title_str.contains(":") {
			let split_title :Vec<&str>= chap_title_str.split(":").collect();
			title = String::from(split_title[1].trim());
		}
		
		let date_str = chapter_obj.select(".meta_r").text().read();
		let date_str_split :Vec<&str>= date_str.split(",").collect();
		let scanlator = String::from(date_str_split[0].replace("par", "").trim());

		let mut date_updated = StringRef::from(&date_str_split[1].trim())
			.0
			.as_date("YYYY.MM.d", Some("fr"), None)
			.unwrap_or(-1.0);
		if date_updated < -1.0 {
			date_updated = StringRef::from(&date_str)
				.0
				.as_date("YYYY.MM.d", Some("fr"), None)
				.unwrap_or(-1.0);
		}
		if date_updated == -1.0 {
			date_updated = current_date();
		}

		chapters.push(Chapter{
			id,
			title,
			volume,
			chapter,
			date_updated,
			scanlator,
			url,
			lang: String::from("fr"),
		});
	}

	Ok(chapters)
}

// parse all images of chapter
pub fn parse_chapter_details(chapter_details_obj: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let data = chapter_details_obj.select("#content > script:nth-child(5)").html().read().lines().enumerate().nth(1).expect("var page").1.trim().replace("var pages =", "").replace("];", "]");
	let json = json::parse(data).unwrap().as_array()?;

	let mut i = 0;
	for item in json {
		let obj = item.as_object()?;
		let url = obj.get("url").as_string()?.read();

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
