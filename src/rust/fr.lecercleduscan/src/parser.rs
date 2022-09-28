use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		html::Node,
		String, StringRef, Vec, current_date
	},
	Manga, Page, MangaStatus, MangaContentRating, MangaViewer, Chapter
};
use regex::Regex;

//////////////////////////
//// PARSER FUNCTIONS ////
//////////////////////////

// parse manga with basic details
pub fn parse_mangas(html: Node, mangas: &mut Vec<Manga>) {
	for page in html.select(".group").array() {
		let obj = page.as_node();

		let url = obj.select(">.title a").attr("href").read();
		let split_url :Vec<&str>= url.split("/").collect();
		let id = String::from(split_url[4]);

		let title = obj.select(">.title a").attr("title").read();

		let mut cover :String = String::from(obj.select(".preview").attr("src").read().trim());

		if cover == "" {
			let url = format!("https://lel.lecercleduscan.com/series/{}", &id);
			let html = Request::new(&url, HttpMethod::Get).html();
			cover = String::from(html.select(".thumbnail img").attr("src").read().trim());
		}

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

// check if is last page of list manga
pub fn check_not_last_page(html: Node) -> bool {
	return html.select(".next").text().read().len() != 0;
}

// parse mangas with full details
pub fn parse_manga_details(manga_obj: Node, id: String) -> Result<Manga> {	
	let cover = String::from(manga_obj.select(".thumbnail img").attr("src").read().trim());
	let title = manga_obj.select(".large.comic .title").text().read();

	let re = Regex::new(r"(Artist</b>: ?)([^\n<]*)[\n<]").unwrap();
	let matches = re.find(&manga_obj.select(".large.comic .info").text().read()).unwrap();
	println!("{}",matches.as_str());
	
	let author = String::new();
	let artist = String::new();
	let description = String::new();
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
		let chapter_obj = chapter.as_node();

		let url = chapter_obj.select(".title a").attr("href").read();
		let id = String::from(&url[36..]);

		let split_url :Vec<&str>= url.split("/").collect();
		let volume = if split_url[6] == "0" {
			-1.0
		} else {
			String::from(split_url[6]).parse().unwrap()
		} as f32;

		let chap_title_str = chapter_obj.select(".title a").text().read();
		let mut title = String::new();
		if chap_title_str.contains(":") {
			let split_title :Vec<&str>= chap_title_str.split(":").collect();
			title = String::from(split_title[1].trim());
		}
		let split_str :Vec<&str>= chap_title_str.split(" ").collect();
		let chapter = String::from(split_str[1]).replace(":", "").parse().unwrap();

		let date_str = chapter_obj.select(".meta_r").text().read();
		let date_str_split :Vec<&str>= date_str.split(",").collect();

		let scanlator = String::from(date_str_split[0].replace("par", "").trim());


		// let _matcher: Matcher2<_> = regex!(br"(Artist</b>: ?)([^\n<]*[\n<])");
		// let aritis = format!("b\"{}\"", chapter_obj.select(".large.comic .info").text().read());
		// let (_prefix, _suffix) = _matcher.match_slices(aritis.as_bytes()).unwrap();
		// println!("ar{}", String::from_utf8_lossy(_prefix));
		// println!("ra{}", String::from_utf8_lossy(_suffix));
		// println!("test");

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

	let mut i = 0;
	for page in chapter_details_obj.select(".container .reading-content img").array() {
		let chapter_details_obj = page.as_node();
		//let mut url = String::from(obj.attr("data-src").read().trim());

		let url = if String::from(chapter_details_obj.attr("src").read().trim()) == "" {
			String::from(chapter_details_obj.attr("data-src").read().trim())
		} else {
			String::from(chapter_details_obj.attr("src").read().trim())
		};

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




// pub fn parse_incoming_url(url: String) -> String {
//     // https://mangapill.com/manga/6290/one-piece-pirate-recipes
//     // https://mangapill.com/chapters/6290-10006000/one-piece-pirate-recipes-chapter-6

//     let split = url.as_str().split("/");
//     let vec = split.collect::<Vec<&str>>();
//     let mut manga_id = String::from("/manga/");

//     if url.contains("/chapters/") {
//         let split  = vec[vec.len() - 2].split("-");
//         let ch_vec = split.collect::<Vec<&str>>();
//         manga_id.push_str(ch_vec[0]);
//     } else {
//         manga_id.push_str(vec[vec.len() - 2]);
//     }
//     manga_id.push_str("/");
//     manga_id.push_str(vec[vec.len() - 1]);
//     return manga_id;
// }