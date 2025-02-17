#![no_std]
use aidoku::{
	error::Result, prelude::*, std::net::Request, std::String, std::Vec, Chapter, DeepLink, Filter,
	Listing, Manga, MangaPageResult, Page,
};

use mangastream_template::template::MangaStreamSource;

fn get_instance() -> MangaStreamSource {
	MangaStreamSource {
		base_url: String::from("https://sushiscan.net"),
		listing: ["Dernières", "Populaire", "Nouveau"],
		status_options: ["En Cours", "Terminé", "En Pause", "", "Abandonné"],
		traverse_pathname: "catalogue",
		last_page_text: "Suivant",
		last_page_text_2: "Next",
		status_options_search: ["", "ongoing", "completed", "hiatus", "en pause"],
		type_options_search: ["", "manga", "manhwa", "manhua", "comic", "fanfiction", "bande-dessinée", "global-manga", "artbook", "anime-comics", "guidebook", ""],
		manga_details_categories: ".seriestugenre a",
		manga_details_author: ".infotable td:contains(Auteur)+td",
		manga_details_artist: ".infotable td:contains(Dessinateur)+td",
		manga_details_status: ".infotable td:contains(Statut)+td",
		manga_details_type: ".infotable td:contains(Type)+td",
		chapter_date_format : "MMMM dd, yyyy",
		language: "fr",
		locale: "fr-FR",
		alt_pages: true,
		..Default::default()
	}
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	get_instance().parse_manga_list(filters, page)
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	get_instance().parse_manga_listing(get_instance().base_url, listing.name, page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	get_instance().parse_manga_details(id)
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	get_instance().parse_chapter_list(id)
}

#[get_page_list]
fn get_page_list(_manga_id: String, id: String) -> Result<Vec<Page>> {
	get_instance().parse_page_list(id)
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	get_instance().modify_image_request(request)
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	get_instance().handle_url(url)
}
