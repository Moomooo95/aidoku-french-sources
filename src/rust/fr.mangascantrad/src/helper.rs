use aidoku::{
	std::{
		html::Node,
		String, Vec
	}
};

//////////////////////////
//// HELPER FUNCTIONS ////
//////////////////////////

// cast i32 to string
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

// encode url
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

// get best url img of image
pub fn get_url_image(image_obj: Node) -> String {
	let mut image = String::new();

	for attr in ["srcset", "data-srcset", "data-src", "src"] {
		if image.len() != 0 { break; }

		let url = String::from(image_obj.attr(attr).read().trim());
		if url == "" { continue; }

		if attr.contains("srcset") {
			let split_one :Vec<&str> = url.split(",").collect();
			let split_two :Vec<&str> = split_one[split_one.len()-1].trim().split(" ").collect();
			image = String::from(split_two[0]);
		} else if !url.contains("dflazy") { 
			image = url;
		}
	}

	return remove_size_url_image(image);
}

// remove size of url image of a manga
pub fn remove_size_url_image(url: String) -> String {
	for pattern in ["-125x180.", "-193x278.", "-110x150.", "-150x206.", "-75x106."] {
		if url.contains(pattern) {
			return url.replace(pattern, ".");
		}
	}

	return url;
}
