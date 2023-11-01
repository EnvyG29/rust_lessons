fn main() {
	let width1: u32 = input().trim().parse().expect("err_parse");
	let height1: u32 = input().trim().parse().expect("err_parse");

	let rect1 = Rectangle {
		width: width1, 
		height: height1
	};

	let rect2 = Rectangle {
		width: 10,
		height: 40	
	};

	let rect3 = Rectangle {
		width: 60,
		height: 45
	};

	let sq = Rectangle::square(width1);

	dbg!(&rect1);
	println!("debud {:#?}", rect1);

	println!("...{}...", fn2(&rect1));

	println!("__{}__", rect1.area());

	println!("~~{}~~", sq.area());

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}



#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32
}

impl Rectangle {
	fn area (&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

impl Rectangle {	
	fn square(size:u32) -> Self {
		Self {
			width: size,
			height: size
		}
	}
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

