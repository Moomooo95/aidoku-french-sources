use aidoku::{
	error::Result, prelude::*, std::{
		current_date, ObjectRef, String, StringRef, Vec
	}, Chapter, Manga, MangaContentRating, MangaPageResult, MangaStatus, MangaViewer, Page
};

use crate::{helper::i32_to_string, BASE_URL};
use crate::API_URL;

pub fn parse_manga_list(json: ObjectRef) -> Result<MangaPageResult>  {
	let mut mangas: Vec<Manga> = Vec::new();
	
	for item in json.get("mangas").as_array()? {
		let manga = item.as_object()?;

		if manga.get("slug").as_string()?.read() == "unknown" {
			continue
		}
		
		let id = manga.get("slug").as_string()?.read();
		let title = manga.get("title").as_string()?.read();
		let cover = format!("{}/{}", String::from(API_URL), manga.get("coverImage").as_string()?.read());
		let status_str = manga.get("status").as_string()?.read();
		let status = match status_str.as_str() {
			"Ongoing" => MangaStatus::Ongoing,
			"Completed" => MangaStatus::Completed,
			"Hiatus" => MangaStatus::Hiatus,
			_ => MangaStatus::Unknown,
		};
		let manga_type = manga.get("type").as_string()?.read();
		let viewer = match manga_type.as_str() {
			"Manga" => MangaViewer::Rtl,
			_ => MangaViewer::Scroll,
		};

		mangas.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url: String::new(),
			categories: Vec::new(),
			status,
			nsfw: MangaContentRating::Safe,
			viewer
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: json.get("pagination").as_object()?.get("hasNextPage").as_bool()?,
	})
}

pub fn parse_search_list(json: ObjectRef) -> Result<MangaPageResult>  {
	let mut mangas: Vec<Manga> = Vec::new();
	
	for item in json.get("mangas").as_array()? {
		let manga = item.as_object()?;

		if manga.get("slug").as_string()?.read() == "unknown" {
			continue
		}
		
		let id = manga.get("slug").as_string()?.read();
		let title = manga.get("title").as_string()?.read();
		let cover = format!("{}/{}", String::from(API_URL), manga.get("coverImage").as_string()?.read());
		let manga_type = manga.get("type").as_string()?.read();
		let viewer = match manga_type.as_str() {
			"Manga" => MangaViewer::Rtl,
			_ => MangaViewer::Scroll,
		};

		mangas.push(Manga {
			id,
			cover,
			title,
			author: String::new(),
			artist: String::new(),
			description: String::new(),
			url: String::new(),
			categories: Vec::new(),
			status: MangaStatus::Unknown,
			nsfw: MangaContentRating::Safe,
			viewer
		});
	}

	Ok(MangaPageResult {
		manga: mangas,
		has_more: false,
	})
}

pub fn parse_manga_details(manga_id: String, json: ObjectRef) -> Result<Manga> {	
	let manga = json.get("manga").as_object()?;

	// Récupérer l'image de couverture
	let cover = format!("{}/{}", String::from(API_URL), manga.get("coverImage").as_string()?.read());
	
	// Récupérer le titre
	let title = manga.get("title").as_string()?.read();

	// Récupérer la description (avec valeur par défaut)
	let description = if manga.get("synopsis").is_some() && !manga.get("synopsis").as_string()?.read().is_empty() {
		manga.get("synopsis").as_string()?.read()
	} else {
		String::from("Aucune description disponible.")
	};

	// Récupérer l'URL
	let url = format!("{}/manga/{}", String::from(BASE_URL), manga_id);

	// Récupérer le statut du manga
	let status_str = manga.get("status").as_string()?.read();
	let status = match status_str.as_str() {
		"Ongoing" => MangaStatus::Ongoing,
		"Completed" => MangaStatus::Completed,
		"Hiatus" => MangaStatus::Hiatus,
		_ => MangaStatus::Unknown,
	};

    // Récupérer les catégories (genres)
	let mut categories: Vec<String> = Vec::new();
	for item in manga.get("genres").as_array()? {
		let genre = item.as_object()?;
		categories.push(genre.get("name").as_string()?.read());
	}

	// Récupérer le type de manga
	let manga_type = manga.get("type").as_string()?.read();
	let viewer = match manga_type.as_str() {
		"Manga" => MangaViewer::Rtl,
		_ => MangaViewer::Scroll,
	};

	Ok(Manga {
		id: manga_id,
		cover,
		title,
		author: String::new(),
		artist: String::new(),
		description,
		url,
		categories,
		status,
		nsfw: MangaContentRating::Safe,
		viewer
	})
}

pub fn parse_chapter_list(manga_id: String, json: ObjectRef) -> Result<Vec<Chapter>> {
	let mut chapters: Vec<Chapter> = Vec::new();
	
	for item in json.get("chapters").as_array()? {
		let chapter_object = item.as_object()?;
		let id = i32_to_string(chapter_object.get("number").as_int()? as i32);
		let chapter = chapter_object.get("number").as_int()? as f32;
		let date_str = chapter_object.get("createdAt").as_string()?.read();
		let mut date_updated = StringRef::from(&date_str)
			.0
			.as_date("yyyy-MM-dd'T'HH:mm:ss.SSSZ", Some("en"), None)
			.unwrap_or(-1.0);
	
		if date_updated == -1.0 {
			date_updated = current_date();
		}
		let url = format!("{}/manga/{}/chapitre/{}", String::from(BASE_URL), manga_id , id);

		chapters.push(Chapter{
			id,
			title: String::from(""),
			volume: -1.0,
			chapter,
			date_updated,
			scanlator: String::from(""),
			url,
			lang: String::from("fr"),
		});
	}

	Ok(chapters)
}

pub fn parse_page_list(json: ObjectRef) -> Result<Vec<Page>> {
	let mut pages: Vec<Page> = Vec::new();

	for (index, item) in json.get("chapter").as_object()?.get("images").as_array()?.enumerate() {
		pages.push(Page {
			index: index as i32,
			url: format!("{}/{}", String::from(API_URL), item.as_string()?.read()),
			base64: String::new(),
			text: String::new(),
		});
	}

	Ok(pages)
}