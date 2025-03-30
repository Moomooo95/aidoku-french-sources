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

pub static BASE_URL: &str = "https://phenix-scans.com";
pub static API_URL: &str = "https://api.phenix-scans.com";

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let manga_per_page = 20;
	let mut query = String::new();
	let mut genres_query: Vec<String> = Vec::new();
	let mut search_query = String::new();
	
	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				if let Ok(value) = filter.value.as_string() {
					search_query = format!("query={}", helper::urlencode(value.read()));
				}
			}
			FilterType::Select => {
				if filter.name == "Status" {
					let index = filter.value.as_int().unwrap_or(-1);
					match index {
						1 => query.push_str("&status=Ongoing"),
						2 => query.push_str("&status=Completed"),
						3 => query.push_str("&status=Hiatus"),
						_ => continue,
					}
				}
				if filter.name == "Type" {
					let index = filter.value.as_int().unwrap_or(-1);
					let value = filter.object.get("options").as_array()?.get(index as usize).as_string()?.read();
					match index {
						0 => continue,
						_ => query.push_str(format!("&type={}", helper::urlencode(value)).as_str()),
					}
				}
				if filter.name == "Trier par" {
					let index = filter.value.as_int().unwrap_or(-1);
					match index {
						0 => query.push_str("&sort=rating"),
						1 => query.push_str("&sort=updatedAt"),
						2 => query.push_str("&sort=title"),
						3 => query.push_str("&sort=chapters"),
						_ => continue,
					}
				}
			}
			FilterType::Genre => {
				if let Ok(filter_id) = filter.object.get("id").as_string() {
					genres_query.push(helper::urlencode(filter_id.read()));
				}
			}
			_ => continue,
		}
	}

	if search_query.is_empty() {
		let genres_query_str = String::from(&genres_query.join(","));
		let url = format!("{}/front/manga?{}&genre={}&page={}&limit={}", String::from(API_URL), query, genres_query_str, helper::i32_to_string(page), helper::i32_to_string(manga_per_page));
		let json = Request::new(&url, HttpMethod::Get).json()?.as_object()?;
		parser::parse_manga_list(json)
	} else {
		let url = format!("{}/front/manga/search?{}", String::from(API_URL), search_query);
		let json = Request::new(&url, HttpMethod::Get).json()?.as_object()?;
		parser::parse_search_list(json)
	}
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let manga_per_page = 20;
	let mut url = String::new();
	if listing.name == "Dernières Sorties" {
		url = format!("{}/front/manga?sort=updatedAt&page={}&limit={}", String::from(API_URL), helper::i32_to_string(page), helper::i32_to_string(manga_per_page));
	} else if listing.name == "Populaire" {
		url = format!("{}/front/manga?sort=rating&page={}&limit={}", String::from(API_URL), helper::i32_to_string(page), helper::i32_to_string(manga_per_page));
	};
	let json = Request::new(&url, HttpMethod::Get).json()?.as_object()?;
	parser::parse_manga_list(json)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = format!("{}/front/manga/{}", String::from(API_URL), manga_id);
	let json = Request::new(url, HttpMethod::Get).json()?.as_object()?;
	parser::parse_manga_details(manga_id, json)
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = format!("{}/front/manga/{}", String::from(API_URL), manga_id);
	let json = Request::new(url, HttpMethod::Get).json()?.as_object()?;
	parser::parse_chapter_list(manga_id, json)
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("{}/front/manga/{}/chapter/{}", String::from(API_URL), manga_id, chapter_id);
	let json = Request::new(url, HttpMethod::Get).json()?.as_object()?;
	parser::parse_page_list(json)
}
