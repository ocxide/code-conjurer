/// Capitalizes the first letter of a given string slice.
///
/// # Examples
///
/// ```
/// let s = "hello";
/// let capitalized = capitalize_once(s);
/// assert_eq!(capitalized, "Hello");
///
/// let s = "";
/// let capitalized = capitalize_once(s);
/// assert_eq!(capitalized, "");
/// ```
pub fn capitalize_once(slice: &str) -> String {
	// Get an iterator over the characters in the input slice
	let mut chars = slice.chars();

	// Get the first character of the slice, if it exists
	let first_letter = match chars.next() {
		Some(c) => c,
		None => return String::with_capacity(0), // Return an empty string if the slice is empty
	};

	// Create a new string to store the capitalized version of the slice
	let mut capitalized = String::new();

	// Capitalize the first letter and add it to the new string
	capitalized.push(first_letter.to_uppercase().next().unwrap());

	// Iterate over the rest of the characters in the slice and add them to the new string
	for c in chars {
		capitalized.push(c);
	}

	// Return the capitalized string
	capitalized
}

pub fn capitalize_all(slice: &str, separator: char) -> String {
	slice.split(separator).map(capitalize_once).collect()
}
