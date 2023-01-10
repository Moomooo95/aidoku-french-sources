#![no_std]
use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		String, Vec, ObjectRef
	},
	Filter, FilterType, Manga, MangaPageResult, Page, Chapter, Listing, MangaStatus, MangaContentRating, MangaViewer
};

mod parser;
mod helper;

use helper::*;

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut url = format!("https://lelscanvf.com/filterList?alpha=&artist=&tag=&page={}", &i32_to_string(page));
	let mut sort = -1;

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				if let Ok(value) = filter.value.as_string() {
					url = String::from("https://lelscanvf.com/search");
					url.push_str("?query=");
					url.push_str(&urlencode(value.read()));
					break;
				}
			}
			FilterType::Author => {
				if let Ok(value) = filter.value.as_string() {
					url.push_str("&author=");
					url.push_str(&urlencode(value.read()));
				}
			}
			FilterType::Genre => {
				if let Ok(id) = filter.object.get("id").as_string() {
					if !url.contains("&cat=") || !url.contains("search") {
						url.push_str("&cat=");
						url.push_str(&id.read());
					}		
				}
			}
			FilterType::Sort => {
				let value = match filter.value.as_object() {
					Ok(value) => value,
					Err(_) => continue,
				};
				let index = value.get("index").as_int().unwrap_or(0);
				if !url.contains("search") || !url.contains("filterList") {
					if index == 0 || index == 1 {
						sort = index;
						url = String::from("https://lelscanvf.com/");
						break;
					} else if index == 2 {
						sort = index;
						url = String::from("https://lelscanvf.com/topManga");
						break;
					} else {
						url.push_str("&sortBy=");
						url.push_str(match index {
							0 => "name",
							1 => "name",
							2 => "name",
							3 => "views",
							4 => "name",
							_ => "name"
						});
					}
				}
				
			}
			FilterType::Select => match filter.name.as_str() {
				"Asc" => {
					url.push_str("&asc=");
					match filter.value.as_int().unwrap_or(-1) {
						0 => url.push_str("true"),
						1 => url.push_str("false"),
						_ => url.push_str("true")
					}
				}
				_ => continue,
			}
			_ => continue,
		}
	}

	let mut manga: Vec<Manga> = Vec::new();
	let mut has_more = false;

	if url.contains("search") {
		let json = Request::new(&url, HttpMethod::Get).json()?.as_object()?;

		for item in json.get("suggestions").as_array()? {
			let data = item.as_object()?;

			let id = data.get("data").as_string()?.read();
			let cover = format!("https://lelscanvf.com/uploads/manga/{}/cover/cover_250x350.jpg", id);
			let title = data.get("value").as_string()?.read();
			let url = format!("https://lelscanvf.com/manga/{}", id);
	
			if id.len() > 0 && title.len() > 0 && cover.len() > 0 {
				manga.push(Manga {
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
	} else if url.contains("filterList") {
		let html = Request::new(&url, HttpMethod::Get).html()?;
		manga = parser::parse_filterlist(html);
		has_more = parser::is_last_page(Request::new(&url, HttpMethod::Get).html()?);
	} else if url.contains("topManga") {
		let html = Request::new(&url, HttpMethod::Get).html()?;
		manga = parser::parse_top_manga(html);
	} else {
		let html = Request::new(&url, HttpMethod::Get).html()?;
		if sort == 0 {
			manga = parser::parse_recents(html);
		} else if sort == 1 {
			manga = parser::parse_recents_popular(html);
		}
	}

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
		selection.set("index", 0.into());
		filters.push(Filter {
			kind: FilterType::Sort,
			name: String::from("Sort"),
			value: selection.0.clone(),
			object: selection,
		});
	} else if listing.name == "Dernières Sorties Populaires" {
		selection.set("index", 1.into());
		filters.push(Filter {
			kind: FilterType::Sort,
			name: String::from("Sort"),
			value: selection.0.clone(),
			object: selection,
		});
	} else if listing.name == "Tendances" {
		selection.set("index", 2.into());
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
	let url = format!("https://lelscanvf.com/manga/{}", &manga_id);
	let html = Request::new(url.clone().as_str(), HttpMethod::Get).html()?;
	return parser::parse_manga(html, manga_id);
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = format!("https://lelscanvf.com/manga/{}", &manga_id);
	let html = Request::new(url.clone().as_str(), HttpMethod::Get).html()?;
	return parser::get_chaper_list(html);
}

#[get_page_list]
fn get_page_list(manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("https://lelscanvf.com/manga/{}/{}", &manga_id, &chapter_id);
	let html = Request::new(url.clone().as_str(), HttpMethod::Get).header("Referer", "https://lelscanvf.com/").html()?;
	return parser::get_page_list(html);
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	request.header("Referer", "https://lelscanvf.com/");
}