#![no_std]
use aidoku::{
	error::Result, prelude::*, std::net::Request, std::String, std::Vec, Chapter, DeepLink, Filter, Listing, Manga,
	MangaPageResult, MangaStatus, Page,
};
use madara_template::template;

fn get_data() -> template::MadaraSiteData {
	let data: template::MadaraSiteData = template::MadaraSiteData {
		base_url: String::from("https://manga-scantrad.io"),
		lang: String::from("fr"),
		date_format: String::from("d MMMM yyyy"),
		status_filter_ongoing: String::from("En cours"),
		status_filter_completed: String::from("Terminé"),
		status_filter_cancelled: String::from("Annulé"),
		status_filter_on_hold: String::from("En pause"),
		popular: String::from("Populaire"),
		trending: String::from("Tendance"),
		status: |html| {
			let status_str = html
				.select("div.post-content_item:contains(État) div.summary-content")
				.text()
				.read()
				.trim()
				.to_lowercase();
			match status_str.as_str() {
				"ongoing" | "releasing" => MangaStatus::Ongoing,
				"completed" => MangaStatus::Completed,
				"canceled" | "dropped" => MangaStatus::Cancelled,
				"hiatus" | "on hold" => MangaStatus::Hiatus,
				_ => MangaStatus::Unknown,
			}
		},
		alt_ajax: true,
		user_agent: Some(String::from("Mozilla/5.0 (iPhone; CPU iPhone OS 17_4_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.3.1 Mobile/15E148 Safari/604.1")),
		..Default::default()
	};
	data
}

#[get_manga_list]
fn get_manga_list(filters: Vec<Filter>, page: i32) -> Result<MangaPageResult> {
	template::get_manga_list(filters, page, get_data())
}

#[get_manga_listing]
fn get_manga_listing(listing: Listing, page: i32) -> Result<MangaPageResult> {
	template::get_manga_listing(get_data(), listing, page)
}

#[get_manga_details]
fn get_manga_details(id: String) -> Result<Manga> {
	template::get_manga_details(id, get_data())
}

#[get_chapter_list]
fn get_chapter_list(id: String) -> Result<Vec<Chapter>> {
	template::get_chapter_list(id, get_data())
}

#[get_page_list]
fn get_page_list(_manga_id: String, chapter_id: String) -> Result<Vec<Page>> {
	template::get_page_list(chapter_id, get_data())
}

#[modify_image_request]
fn modify_image_request(request: Request) {
	template::modify_image_request(String::from("manga-scantrad.io"), request, get_data());
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	template::handle_url(url, get_data())
}
