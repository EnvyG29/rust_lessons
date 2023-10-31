fn main() {
    fn1();
	fn2();
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn fn1() {
    let mut user1 = User {
    active: true,
    username: String::from("RePython"),
    email: String::from("mail@mail.com"),
    sign_in_count: 1
    };

    user1.email = String::from("list@mail.org");

    let _user2 = User {
        username: String::from("another_user"),
        ..user1
    };
}


fn fn2 () {
	let black = Color(9, 9, 9);
	let origin = Point(9, 9, 9); 
}



