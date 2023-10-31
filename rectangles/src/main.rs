fn main() {
	let width1: u32 = input().trim().parse().expect("err_parse");
	let height1: u32 = input().trim().parse().expect("err_parse");

	let rect1 = Rectangle {
			width: width1, 
			height: height1
	};

	dbg!(&rect1);
	println!("debud {:#?}", rect1);

	println!("...{}...", fn2(&rect1));

	println!("__{}__", area(width1, height1));

}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

fn fn2(dimensions: &Rectangle) -> u32 {
	dimensions.width * dimensions.height
}

fn input() -> String {
	use std::io;
	
	println!(">>> ");
	
	let mut input = String::new();
	
	io::stdin().read_line(&mut input).expect("err_input");

	input
}

fn area (width: u32, height: u32) -> u32 {
	width * height
}
