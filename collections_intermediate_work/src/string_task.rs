pub fn task() {
	let input: String = input(String::from(">>> "));
	let new_str: String = pig_latin(&input);
	println!("{}", new_str);
}

fn pig_latin(s: &str) -> String {
	let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
	let mut result: String = String::new();

	for word in s.split_whitespace() {
		let first_char: char = word.chars()
			.next()
			.unwrap();
		if vowels.contains(&first_char) {
			result.push_str(&format!("{}-hay ", word));
		} else {
			let rest = &word[first_char.len_utf8()..];
			result.push_str(&format!("{}-{}ay ", rest, first_char));
		}
	}
	result.trim().to_string()
}

fn input(s: String) -> String {
	use std::io;
	println!("{} ", s);
	let mut input: String = String::new();
	io::stdin().read_line(&mut input).expect("err input!!!");
	input
}
