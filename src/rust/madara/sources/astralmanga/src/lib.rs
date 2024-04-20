#![no_std]
use aidoku::{
	error::Result, prelude::*, std::net::Request, std::String, std::Vec, Chapter, DeepLink, Filter, Listing, Manga,
	MangaPageResult, MangaStatus, MangaContentRating, Page,
};
use madara_template::template;

fn get_data() -> template::MadaraSiteData {
	let data: template::MadaraSiteData = template::MadaraSiteData {
		base_url: String::from("https://astral-manga.fr"),
		lang: String::from("fr"),
		description_selector: String::from("div.manga-excerpt p"),
		date_format: String::from("dd/MM/yyyy"),
		status_filter_ongoing: String::from("En cours"),
		status_filter_completed: String::from("Terminé"),
		status_filter_cancelled: String::from("Annulé"),
		status_filter_on_hold: String::from("En attente"),
		popular: String::from("Populaire"),
		trending: String::from("Tendance"),
		status: |html| {
			let status_str = html
				.select("div.post-content_item:contains(Statut) div.summary-content")
				.text()
				.read()
				.to_lowercase();
			match status_str.as_str() {
				"en cours" => MangaStatus::Ongoing,
				"terminé" => MangaStatus::Completed,
				"annulé" => MangaStatus::Cancelled,
				"en attente" => MangaStatus::Hiatus,
				_ => MangaStatus::Unknown,
			}
		},
		nsfw: |html, categories| {
			if !html
				.select(".manga-title-badges.adult")
				.text()
				.read()
				.is_empty()
			{
				MangaContentRating::Safe
			} else {
				let suggestive_tags = ["ecchi", "mature"];

				for tag in suggestive_tags {
					if categories.iter().any(|v| v.to_lowercase() == tag) {
						return MangaContentRating::Suggestive;
					}
				}

				MangaContentRating::Safe
			}
		},
		alt_ajax: true,
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
	template::modify_image_request(String::from("manga-scantrad.io"), request);
}

#[handle_url]
pub fn handle_url(url: String) -> Result<DeepLink> {
	template::handle_url(url, get_data())
}
