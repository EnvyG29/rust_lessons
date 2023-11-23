use std::collections::HashMap;
pub fn fn1(){	
	println!("");
	let mut scores: HashMap<String, i32> = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);
	scores.insert(String::from("Yellow"), 30); // перезапись значения ключа
	scores.entry(String::from("Blue")).or_insert(20);
	scores.entry(String::from("Green")).or_insert(20);
	println!("{:?}\n", scores);	
	println!("{:#?}\n", scores);	

	let team_name = String::from("Blue");
	let score = scores.get(&team_name).copied().unwrap_or(0);
	println!("{}", score);	

	for (key, value) in &scores {
		println!("{key}: {value}");
	}

	
}

pub fn fn2(){
	println!("");

	let text = "hello world wondeful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace(){
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}
	println!("{:#?}", map);
}
