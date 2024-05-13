#![no_std]
use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		html::Node,
		String, Vec
	},
	Filter, FilterType, Manga, MangaPageResult, Page, Chapter
};

mod parser;
mod helper;

const BASE_URL: &str = "https://lelscanfr.com";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut query = String::new();
	
	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				if let Ok(value) = filter.value.as_string() {
					query.push_str(format!("&title={}", helper::urlencode(value.read())).as_str());
				}
			}
			FilterType::Genre => {
				query.push_str("&genre[]=");
				if let Ok(filter_id) = filter.object.get("id").as_string() {
					query.push_str(filter_id.read().as_str());
				}
			}
			FilterType::Select => {
				if filter.name == "Type" {
					match filter.value.as_int().unwrap_or(-1) {
						0 => query.push_str("&type="),
						1 => query.push_str("&type=manga"),
						2 => query.push_str("&type=manhua"),
						3 => query.push_str("&type=manhwa"),
						4 => query.push_str("&type=bande-dessinee"),
						_ => continue,
					}
				}
				if filter.name == "Status" {
					match filter.value.as_int().unwrap_or(-1) {
						0 => query.push_str(""),
						1 => query.push_str("&status=en-cours"),
						2 => query.push_str("&status=termin"),
						_ => continue,
					}
				}
			}
			_ => continue,
		}
	}
	
	let url = format!("{}/manga?page={}{}", String::from(BASE_URL), helper::i32_to_string(page), query);
	let html = Request::new(&url, HttpMethod::Get).html()?;	
	parser::parse_manga_list(html)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = format!("{}/manga/{}", String::from(BASE_URL), manga_id);
	let html = Request::new(url, HttpMethod::Get).html()?;
	parser::parse_manga_details(String::from(BASE_URL), manga_id, html)
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = format!("{}/manga/{}", String::from(BASE_URL), manga_id);
	let html = Request::new(url, HttpMethod::Get).html()?;

	if !html.select(".pagination").text().read().is_empty() {
		let nb_pages = html.select(".pagination-link").last().previous().expect("last page").attr("onclick").read().chars().rev().nth(1).unwrap().to_digit(10).unwrap();
		let mut all_nodes: Vec<Node> = Vec::new();
		for page in 1..nb_pages+1 {
			let url = format!("{}/manga/{}?page={}", String::from(BASE_URL), manga_id, page);
			all_nodes.push(Request::new(url, HttpMethod::Get).html()?)
		}
		parser::parse_chapter_list(String::from(BASE_URL), manga_id, all_nodes)
	} else {
		parser::parse_chapter_list(String::from(BASE_URL), manga_id, [html].to_vec())
	}	
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/manga/{}/{}", String::from(BASE_URL), manga_id, chapter_id);
	let html = Request::new(url, HttpMethod::Get).html()?;
	parser::parse_page_list(html)
}
