use aidoku::{
	std::{
		html::Node,
		String,
		Vec,
	},
};

//////////////////////////
//// HELPER FUNCTIONS ////
//////////////////////////

pub fn i32_to_string(mut integer: i32) -> String {
	if integer == 0 {
		return String::from("0");
	}
	let mut string = String::with_capacity(11);
	let pos = if integer < 0 {
		string.insert(0, '-');
		1
	} else {
		0
	};
	while integer != 0 {
		let mut digit = integer % 10;
		if pos == 1 {
			digit *= -1;
		}
		string.insert(pos, char::from_u32((digit as u32) + ('0' as u32)).unwrap());
		integer /= 10;
	}
	return string;
}

pub fn urlencode(string: String) -> String {
	let mut result: Vec<u8> = Vec::with_capacity(string.len() * 3);
	let hex = "0123456789ABCDEF".as_bytes();
	let bytes = string.as_bytes();

	for byte in bytes {
		let curr = *byte;
		if (b'a'..=b'z').contains(&curr)
			|| (b'A'..=b'Z').contains(&curr)
			|| (b'0'..=b'9').contains(&curr)
		{
			result.push(curr);
		} else {
			result.push(b'%');
			result.push(hex[curr as usize >> 4]);
			result.push(hex[curr as usize & 15]);
		}
	}
	String::from_utf8(result).unwrap_or_default()
}

// get the best url image of a manga
pub fn get_url_image(html: &Node) -> String {
	if html.select("img").attr("srcset").read().trim() != "" {
		let mut url = String::from(html.select("img").attr("srcset").read().trim());
		
		let split_one :Vec<&str> = url.split(",").collect();
		let split_two :Vec<&str> = split_one[split_one.len()-1].trim().split(" ").collect();
		url = String::from(split_two[0]);

		return remove_size_url_image(url);

	} else if html.select("img").attr("data-srcset").read().trim() != "" {
		let mut url = String::from(html.select("img").attr("data-srcset").read().trim());
		
		let split_one :Vec<&str> = url.split(",").collect();
		let split_two :Vec<&str> = split_one[split_one.len()-1].trim().split(" ").collect();
		url = String::from(split_two[0]);

		return remove_size_url_image(url);

	} else if html.select("img").attr("data-src").read().trim() != "" {
		return remove_size_url_image(String::from(html.select("img").attr("data-src").read().trim()));
	} else {
		return remove_size_url_image(String::from(html.select("img").attr("src").read().trim()));
	}
}


// remove size of url image of a manga
pub fn remove_size_url_image(url: String) -> String {
	if url.contains("-125x180.") {
		return url.replace("-125x180.", ".");
	} else if url.contains("-193x278.") {
		return url.replace("-193x278.", ".");		
	} else if url.contains("-110x150.") {
		return url.replace("-110x150.", ".");		
	} else if url.contains("-150x206.") {
		return url.replace("-150x206.", ".");		
	} else if url.contains("-75x106.") {
		return url.replace("-75x106.", ".");		
	} else {
		return url;
	}
}