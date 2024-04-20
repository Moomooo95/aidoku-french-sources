use aidoku::{
	prelude::*,
	error::Result,
	std::{
		html::Node,
		String, Vec
	},
	Manga, Page, MangaPageResult, MangaStatus, MangaContentRating, MangaViewer, Chapter
};

pub fn parse_manga_list(html: Node) -> Result<MangaPageResult>  {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select("div[id=card-real]").array() {
		let page = page
			.as_node()
			.expect("Failed to get data as array of nodes");
		
		let title = page.select("h2").first().text().read();
		let url = page.select("a").attr("href").read();
		let id = String::from(url.split('/').enumerate().nth(4).expect("Failed to get id").1.trim());
		let cover = String::from(page.select("img").attr("data-src").read().trim());

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

	let has_more = html.select(".pagination-disabled[aria-label~=Next]").text().read().is_empty();

	Ok(MangaPageResult {
		manga: mangas,
		has_more,
	})
}

pub fn parse_manga_details(base_url: String, manga_id: String, html: Node) -> Result<Manga> {	
	let cover = String::from(html.select("main img").attr("src").read().trim());
	let title = html.select("main img").attr("alt").read();
	let author = html.select("span:contains(Auteur)+span").text().read();
	let artist = html.select("span:contains(Artiste)+span").text().read();
	let description = if !html.select("#description+p").text().read().is_empty() { 
			html.select("#description+p").text().read() 
		} else { 
			html.select("main .card p").text().read() 
		} as String;
	let url = format!("{}/manga/{}", base_url, manga_id);
	
	Ok(Manga {
		id: manga_id,
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

pub fn parse_chapter_list(base_url: String, manga_id: String, html: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for element in html.select("#chapters-list a").array() {
		let element = element
			.as_node()
			.expect("Failed to get data as array of nodes");

		let url = element.attr("href").read();
		let id = String::from(&url.replace(&format!("{}/manga/{}/", base_url, manga_id), ""));

		let split_url : Vec<&str> = url.split('/').collect();
		let chapter = String::from(split_url[5]).parse().unwrap();

		chapters.push(Chapter{
			id,
			title: String::from(""),
			volume: -1.0,
			chapter,
			date_updated: -1.0,
			scanlator: String::from(""),
			url,
			lang: String::from("fr"),
		});
	}

	Ok(chapters)
}

pub fn parse_page_list(html: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in html
		.select("#chapter-container .chapter-image")
		.array()
		.enumerate()
	{
		pages.push(Page {
			index: index as i32,
			url: item.as_node().expect("node array").attr("data-src").read(),
			base64: String::new(),
			text: String::new(),
		});
	}

	Ok(pages)
}
