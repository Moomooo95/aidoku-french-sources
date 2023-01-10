#![no_std]
use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		String, Vec, ObjectRef
	},
	Filter, FilterType, Manga, MangaPageResult, Page, Chapter, Listing
};

mod parser;
mod helper;

use helper::*;

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut url = format!("https://manga-scantrad.io/?post_type=wp-manga&s&paged={}", &i32_to_string(page));

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				if let Ok(value) = filter.value.as_string() {
					url = url.replace("&s", "");
					url.push_str("&s=");
					url.push_str(&urlencode(value.read()));
				}
			}
			FilterType::Author => {
				if let Ok(value) = filter.value.as_string() {
					url.push_str("&author=");
					url.push_str(&urlencode(value.read()));
				}
			}
			FilterType::Check => {
				if let Ok(id) = filter.object.get("id").as_string() {
					url.push_str("&status[]=");
					url.push_str(&id.read());
				}
			}
			FilterType::Genre => {
				if let Ok(id) = filter.object.get("id").as_string() {
					url.push_str("&genre[]=");
					url.push_str(&id.read());
				}
			}
			FilterType::Sort => {
				let value = match filter.value.as_object() {
					Ok(value) => value,
					Err(_) => continue,
				};
				let index = value.get("index").as_int().unwrap_or(0);
				//let ascending = value.get("ascending").as_bool().unwrap_or(false);
				url.push_str("&m_orderby");
				url.push_str(match index {
					0 => "",
					1 => "=latest",
					2 => "=alphabet",
					3 => "=rating",
					4 => "=trending",
					5 => "=views",
					6 => "=new-manga",
					_ => ""
				});
			}
			FilterType::Select => match filter.name.as_str() {
				"Contenu pour Adulte" => {
					url.push_str("&adult=");
					match filter.value.as_int().unwrap_or(-1) {
						0 => url.push_str(""),
						1 => url.push_str("0"),
						2 => url.push_str("1"),
						_ => url.push_str("")
					}
				}
				"Condition pour les genres" => {
					url.push_str("&op=");
					match filter.value.as_int().unwrap_or(-1) {
						0 => url.push_str(""),
						1 => url.push_str("1"),
						_ => url.push_str(""),
					}
				}
				_ => continue,
			},
			_ => continue,
		}
	}

	let html = Request::new(&url, HttpMethod::Get).html()?;

	let manga: Vec<Manga> = parser::parse_catalogue(html);
	let has_more = parser::is_not_last_pages_catalogue(Request::new(&url, HttpMethod::Get).html()?);
	
	Ok(MangaPageResult {
		manga,
		has_more,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut filters: Vec<Filter> = Vec::with_capacity(1);
	let mut selection = ObjectRef::new();

	if listing.name == "Dernières Sorties" {
		selection.set("index", 1.into());
		filters.push(Filter {
			kind: FilterType::Sort,
			name: String::from("Sort"),
			value: selection.0.clone(),
			object: selection,
		});
	} else if listing.name == "Les plus vus" {
		selection.set("index", 5.into());
		filters.push(Filter {
			kind: FilterType::Sort,
			name: String::from("Sort"),
			value: selection.0.clone(),
			object: selection,
		});
	} else if listing.name == "Nouveau" {
		selection.set("index", 6.into());
		filters.push(Filter {
			kind: FilterType::Sort,
			name: String::from("Sort"),
			value: selection.0.clone(),
			object: selection,
		});
	}

	get_manga_list(filters, page)
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = format!("https://manga-scantrad.io/manga/{}", &manga_id);
	let html = Request::new(&url, HttpMethod::Get).html()?;
	return parser::parse_manga_details(html, manga_id);
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = format!("https://manga-scantrad.io/manga/{}/ajax/chapters/", &manga_id);
	let html = Request::new(url.clone().as_str(), HttpMethod::Post).html()?;
	return parser::parse_chapter_list(html);
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("https://manga-scantrad.io/manga/{}/{}?style=list", &manga_id, &chapter_id);
	let html = Request::new(url.clone().as_str(), HttpMethod::Get).html()?;
	return parser::parse_chapter_details(html);
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	if request.url().read().contains("stockage.manga-scantrad.io") {
		request.header("Host", "stockage.manga-scantrad.io");
	} else if request.url().read().contains("manga-scantrad.io") {
		request.header("Host", "manga-scantrad.io");
	}
}