use aidoku::{
	error::Result, prelude::*, std::{
		current_date, html::Node, ArrayRef, ObjectRef, String, StringRef, Vec
	}, Chapter, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Page
};

use crate::BASE_URL;
use crate::API_URL;

pub fn parse_search_list(json: ObjectRef) -> Result<MangaPageResult>  {
	let mut mangas: Vec<Manga> = Vec::new();
	
	for item in json.get("results").as_array()? {
		let manga = item.as_object()?;
		
		let id = manga.get("slug").as_string()?.read();
		let title = manga.get("title").as_string()?.read();

		mangas.push(Manga {
			id,
			cover: String::new(),
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url: String::new(),
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Rtl
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: false,
	})
}

pub fn parse_comics_list(json: ObjectRef) -> Result<MangaPageResult>  {
	let mut mangas: Vec<Manga> = Vec::new();
	
	for item in json.get("comics").as_array()? {
		let manga = item.as_object()?;
		
		let id = manga.get("slug").as_string()?.read();
		let title = manga.get("title").as_string()?.read();
		let cover = format!("{}/{}", String::from(API_URL), manga.get("cover").as_string()?.read());

		mangas.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url: String::new(),
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Rtl
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: json.get("comics").as_array()?.len() == 20,
	})
}

pub fn parse_comics_listing(json: ArrayRef) -> Result<MangaPageResult>  {
	let mut mangas: Vec<Manga> = Vec::new();
	
	for item in json.clone() {
		let manga = item.clone().as_object()?;
		
		let id = manga.get("slug").as_string()?.read();
		let title = manga.get("title").as_string()?.read();
		let cover = format!("{}/{}", String::from(API_URL), manga.get("cover").as_string()?.read());

		mangas.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url: String::new(),
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer: MangaViewer::Rtl
		});
	}


	Ok(MangaPageResult {
		manga: mangas,
		has_more: json.len() == 28,
	})
}

pub fn parse_manga_details(manga_id: String, html: Node) -> Result<Manga> {	
	let cover = String::from(html.select(".serieImg img").attr("src").read().trim());
	let title = html.select(".serieTitle h1").text().read();
	let author = html.select(".serieAdd p:contains(Auteur) strong").text().read();
	let artist = html.select(".serieAdd p:contains(produit) strong").text().read();
	let description = html.select(".serieDescription p").text().read();
	let url = format!("{}/comics/{}", String::from(BASE_URL), manga_id);

	let mut categories: Vec<String> = Vec::new();
	for item in html.select(".serieGenre span").array() {
		categories.push(item.as_node()?.text().read());
	}
	
	Ok(Manga {
		id: manga_id,
		cover,
		title,
		author,
		artist,
		description,
		url,
		categories,
		status: MangaStatus::Unknown,
		nsfw: MangaContentRating::Safe,
		viewer: MangaViewer::Scroll
	})
}

pub fn parse_chapter_list(html: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for item in html.select(".chapterList a").array() {
		let item = item.as_node()?;

		let url = format!("{}{}", String::from(BASE_URL), item.attr("href").read());
		
		let split_url : Vec<&str> = url.split('/').collect();
		let id = String::from(split_url[6]);
		let chapter = id.parse().unwrap();

		let date_str = item.select("span").last().text().read();
		let mut date_updated = StringRef::from(&date_str)
			.0
			.as_date("dd/MM/yyyy", Some("fr"), None)
			.unwrap_or(-1.0);
	
		if date_updated == -1.0 {
			date_updated = current_date();
		}

		chapters.push(Chapter{
			id,
			title: String::from(""),
			volume: -1.0,
			chapter,
			date_updated,
			scanlator: String::from(""),
			url,
			lang: String::from("fr"),
		});
	}

	Ok(chapters)
}

pub fn parse_page_list(html: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html.select(".readerMainContainer img").array().enumerate()
	{
		pages.push(Page {
			index: index as i32,
			url: item.as_node()?.attr("src").read(),
			base64: String::new(),
			text: String::new(),
		});
	}

	Ok(pages)
}
