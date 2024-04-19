#![no_std]
use mmrcms_template::{mmrcms, template::MMRCMSSource};

mmrcms! {
	MMRCMSSource {
		base_url: "http://mangascan-fr.com",
		lang: "fr",
		category_mapper: |idx| {
			String::from(match idx {
				1 => "1", // Action
				2 => "2", // Adventure
				3 => "3", // Comédie
				4 => "4", // Doujinshi
				5 => "5", // Drame
				6 => "6", // Ecchi
				7 => "7", // Fantasy
				8 => "8", // Webtoon
				9 => "9", // Harem
				10 => "10", // Historique
				11 => "11", // Horreur
				12 => "12", // Thriller
				13 => "13", // Arts Martiaux
				14 => "14", // Mature
				15 => "15", // Tragique
				16 => "16", // Mystère
				17 => "17", // One Shot
				18 => "18", // Psychologique
				19 => "19", // Romance
				20 => "20", // School Life
				21 => "21", // Science-fiction
				22 => "22", // Seinen
				23 => "23", // Erotique
				24 => "24", // Shoujo Ai
				25 => "25", // Shounen
				26 => "26", // Shounen Ai
				27 => "27", // Slice of Life
				28 => "28", // Sports
				29 => "29", // Surnaturel
				30 => "30", // Tragedy
				31 => "31", // Gangster
				32 => "32", // Crime
				33 => "33", // Biographique
				34 => "34", // Fantastique
				_ => "",
			})
		},
		..Default::default()
	}
}
