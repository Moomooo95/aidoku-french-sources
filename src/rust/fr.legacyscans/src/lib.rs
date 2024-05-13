#![no_std]
use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		String, Vec
	},
	Filter, FilterType, Listing, Manga, MangaPageResult, Page, Chapter
};

mod parser;
mod helper;

pub static BASE_URL: &str = "https://legacy-scans.com";
pub static API_URL: &str = "https://api.legacy-scans.com";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let manga_per_page = 20;
	let end = manga_per_page*page;
	let start = (end - manga_per_page) + 1;
	let mut query = String::new();
	let mut genres_query = String::new();
	let mut title_query = String::new();
	
	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				if let Ok(value) = filter.value.as_string() {
					title_query = format!("title={}", helper::urlencode(value.read()));
				}
			}
			FilterType::Select => {
				if filter.name == "Status" {
					let index = filter.value.as_int().unwrap_or(-1);
					let value = filter.object.get("options").as_array()?.get(index as usize).as_string()?.read();
					match index {
						0 => query.push_str("&status="),
						1 | 2 | 3 | 4 | 5 => query.push_str(format!("&status={}", helper::urlencode(value)).as_str()),
						_ => continue,
					}
				}
				if filter.name == "Type" {
					let index = filter.value.as_int().unwrap_or(-1);
					let value = filter.object.get("options").as_array()?.get(index as usize).as_string()?.read();
					match index {
						0 => query.push_str("&type="),
						1 | 2 | 3 | 4 => query.push_str(format!("&type={}", helper::urlencode(value)).as_str()),
						_ => continue,
					}
				}
				if filter.name == "Ordre" {
					let index = filter.value.as_int().unwrap_or(-1);
					let value = filter.object.get("options").as_array()?.get(index as usize).as_string()?.read();
					match index {
						0 => query.push_str("&order="),
						1 | 2 => query.push_str(format!("&order={}", value).as_str()),
						_ => continue,
					}
				}
			}
			FilterType::Genre => {
				if let Ok(filter_id) = filter.object.get("id").as_string() {
					genres_query.push_str(format!("{},", helper::urlencode(filter_id.read())).as_str());
				}
			}
			_ => continue,
		}
	}
	
	if !title_query.is_empty() {
		let url = format!("{}/misc/home/search?{}", String::from(API_URL), title_query);
		let json = Request::new(&url, HttpMethod::Get).json()?.as_object()?;
		parser::parse_search_list(json)
	} else {
		let url = format!("{}/misc/comic/search/query?{}&genreNames={}&start={}&end={}", String::from(API_URL), query, genres_query, helper::i32_to_string(start), helper::i32_to_string(end));
		let json = Request::new(&url, HttpMethod::Get).json()?.as_object()?;
		parser::parse_comics_list(json)
	}
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let manga_per_page = 28;
	let end = manga_per_page*page;
	let start = (end - manga_per_page) + 1;
	let mut url = String::new();
	if listing.name == "Dernières Sorties" {
		url = format!("{}/misc/comic/home/updates?start={}&end={}", String::from(API_URL), helper::i32_to_string(start), helper::i32_to_string(end));
	} else if listing.name == "Populaire (Jour)" {
		url = format!("{}/misc/views/daily", String::from(API_URL));
	} else if listing.name == "Populaire (Mois)" {
		url = format!("{}/misc/views/monthly", String::from(API_URL));
	} else if listing.name == "Populaire (Tous)" {
		url = format!("{}/misc/views/all", String::from(API_URL));
	}
	let json = Request::new(url, HttpMethod::Get).header("Referer", String::from(BASE_URL).as_str()).json()?.as_array()?;
	parser::parse_comics_listing(json)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = format!("{}/comics/{}", String::from(BASE_URL), manga_id);
	let html = Request::new(url, HttpMethod::Get).html()?;
	parser::parse_manga_details(manga_id, html)
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = format!("{}/comics/{}", String::from(BASE_URL), manga_id);
	let html = Request::new(url, HttpMethod::Get).html()?;
	parser::parse_chapter_list(html)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/comics/{}/chapter/{}", String::from(BASE_URL), manga_id, chapter_id);
	let html = Request::new(url, HttpMethod::Get).html()?;
	parser::parse_page_list(html)
}
