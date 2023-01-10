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

// parse latest manga with basic details
pub fn parse_recents(html: Node) -> Vec<Manga>  {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select(".mangalist .manga-item").array() {
		let obj = page.as_node().expect("page");

		let title = obj.select(".manga-heading a").text().read();
		let url = obj.select(".manga-heading a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(4).expect("id").1.trim());
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

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

// parse latest popular manga with basic details
pub fn parse_recents_popular(html: Node) -> Vec<Manga>  {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select(".hot-thumbnails li").array() {
		let obj = page.as_node().expect("page");

		let title = obj.select(".manga-name a").text().read();
		let url = obj.select(".manga-name a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(4).expect("id").1.trim());
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

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

// parse top manga with basic details
pub fn parse_top_manga(html: Node) -> Vec<Manga>  {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select("li").array() {
		let obj = page.as_node().expect("page");

		let title = obj.select("h5 strong").text().read();
		let url = obj.select(".media-left a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(4).expect("id").1.trim());
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

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

// parse manga with basic details
pub fn parse_filterlist(html: Node) -> Vec<Manga>  {
	let mut mangas: Vec<Manga> = Vec::new();

	for page in html.select(".media").array() {
		let obj = page.as_node().expect("page");

		let title = obj.select(".media-heading .chart-title").text().read();
		let url = obj.select(".media-heading .chart-title").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(4).expect("id").1.trim());
		let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);

		if id.len() > 0 && title.len() > 0 && cover.len() > 0 {
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
	}

	return mangas;
}

// check if is last page of list manga
pub fn is_last_page(obj: Node) -> bool {
	return obj.select(".pagination li").last().has_class("disabled")
}

// parse mangas with full details
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
		.for_each(|tag| categories.push(tag.as_node().expect("page").text().read()));

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

// parse all chapter of manga
pub fn get_chaper_list(obj: Node) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();

	for chapter in obj.select(".chapters li:not(.volume)").array() {
		let obj = chapter.as_node().expect("page");

		let title = obj.select(".chapter-title-rtl em").text().read();
		let url = obj.select(".chapter-title-rtl a").attr("href").read();
		let id = String::from(url.split("/").enumerate().nth(5).expect("id").1.trim());
		let chapter = String::from(url.split("/").enumerate().nth(5).expect("chapter num").1.trim()).parse().unwrap();

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

// parse all images of chapter
pub fn get_page_list(obj: Node) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	let mut i = 0;
	for page in obj.select("#all img").array() {
		let obj = page.as_node().expect("page");

		let mut url = String::from(obj.attr("data-src").read().trim());
		let text = String::from(obj.attr("alt").read().trim());

		if !url.contains("https:") {
			url = format!("https:{}", obj.attr("data-src").read().trim());	
		}

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
