pub fn fn1(){
	let hello = "Здравствуйте";
	let len = hello.len();
	let s = &hello[0..4];
	println!(">>> {len} {s}");

	let word = "hello Rust";
	let len = word.len();
	println!(">>> {len}");
}

pub fn fn2(){
	println!("");
	for c in "Зд".chars() {
    	println!("{c}");
	}

	for b in "Зд".bytes() {
    	println!("{b}");
	}
}
