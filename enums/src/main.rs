fn main() {
//	fn1();
//	fn2();
//	fn3();
//	fn4();
	fn5();
	fn6();
}


fn fn1() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn fn2(){
	enum IpAddr {
		V4(String),
		V6(String)
	}

	let home = IpAddr::V4(String::from("127.0.0.1"));
	let loopback = IpAddr::V6(String::from("::1"));
}


fn fn3(){
	enum IpAddr {
		V4(u8,u8,u8,u8),
		V6(String)
	}

	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));

}

fn fn4(){
	enum Massage {
		Quit,
		Move {x:i32, y:i32},
		Write(String),
		ChangeColor(i32, i32, i32)
	}

	impl Massage{
		fn call(&self) {
			todo!();
		}
	}

	let m = Massage::Write(String::from("rust"));

	m.call();
}

fn fn5(){
	value_in_cents(Coin::Quarter(UsState::Alaska));
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("\nState qwartet from {:?}\n", state);
			25
		}
	}
}


fn fn6() {
	fn plus_one(x: Option<i32>) {
		match x {
			Some(p) => println!("has value {}", p+1),
	        None => println!("has no value")
		}
	}

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
}
