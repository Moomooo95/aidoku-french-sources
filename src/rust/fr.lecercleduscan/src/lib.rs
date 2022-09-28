#![no_std]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(int_roundings)]
#![feature(allocator_api)]
use aidoku::{
	prelude::*,
	error::Result,
	std::{
		net::{Request,HttpMethod},
		String, Vec
	},
	Filter, FilterType, Manga, MangaPageResult, Page, Chapter, Listing
};

mod parser;
mod helper;

use helper::*;

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	let mut url = format!("https://lel.lecercleduscan.com/directory/{}", &i32_to_string(page));
	let mut html = Request::new(&url, HttpMethod::Get).html();	
	let mut search = String::new();

	for filter in filters {
		match filter.kind {
			FilterType::Title => {
				if let Ok(value) = filter.value.as_string() {
					search = urlencode(value.read());
				}
			},
			_ => continue,
		}
	}

	if search != "" {
		url = String::from("https://lel.lecercleduscan.com/search/");
		html = Request::new(&url, HttpMethod::Post)
			.header("Host", "lel.lecercleduscan.com")
			.header("Content-Type", "application/x-www-form-urlencoded")
			.header("Content-Length", "11")
			.body(format!("search={}", search).as_bytes())
			.html();		
	}

	let mut mangas: Vec<Manga> = Vec::new();
	parser::parse_mangas(html, &mut mangas);

	let html2 = Request::new(&url, HttpMethod::Get).html();
	let not_last_page = parser::check_not_last_page(html2);	
	
	Ok(MangaPageResult {
		manga: mangas,
		has_more: not_last_page,
	})
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	let mut mangas: Vec<Manga> = Vec::new();
	let mut not_last_page = false;

	if listing.name == "Dernières Sorties" {
		let url = format!("https://lel.lecercleduscan.com/latest/{}", &i32_to_string(page));

		let html = Request::new(&url, HttpMethod::Get).html();
		parser::parse_mangas(html, &mut mangas);

		let html2 = Request::new(&url, HttpMethod::Get).html();
		not_last_page = parser::check_not_last_page(html2);
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: not_last_page,
	})
}

#[get_manga_details]
fn get_manga_details(manga_id: String) -> Result<Manga> {
	let url = format!("https://lel.lecercleduscan.com/series/{}", &manga_id);
	let html = Request::new(&url, HttpMethod::Get).html();
	return parser::parse_manga_details(html, manga_id);
}

#[get_chapter_list]
fn get_chapter_list(manga_id: String) -> Result<Vec<Chapter>> {
	let url = format!("https://lel.lecercleduscan.com/series/{}", &manga_id);
	let html = Request::new(&url, HttpMethod::Post).html();
	return parser::parse_chapter_list(html);
}

#[get_page_list]
fn get_page_list(chapter_id: String) -> Result<Vec<Page>> {
	let url = format!("https://lel.lecercleduscan.com/read/{}", &chapter_id);
	let html = Request::new(&url, HttpMethod::Get).html();
	return parser::parse_chapter_details(html);
}
