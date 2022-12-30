#![no_std]
use aidoku::{
    error::Result,
    prelude::*,
    std::{String, Vec},
    Chapter, Filter, Listing, Manga, MangaPageResult, Page,
};

#[get_manga_list]
fn get_manga_list(_filters: Vec<Filter>, _page: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_listing]
fn get_manga_listing(_listing: Listing, _page: i32) -> Result<MangaPageResult> {
    todo!()
}

#[get_manga_details]
fn get_manga_details(_manga_id: String) -> Result<Manga> {
    todo!()
}

#[get_chapter_list]
fn get_chapter_list(_manga_id: String) -> Result<Vec<Chapter>> {
    todo!()
}

#[get_page_list]
fn get_page_list(_manga_id: String, _chapter_id: String) -> Result<Vec<Page>> {
    todo!()
}