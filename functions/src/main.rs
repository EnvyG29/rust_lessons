fn main() {
    another_function(4, 'k');
    let five = five();
    let n = plus_one(6);
    println!("{five} and {n}");
}

fn another_function(x: i32, y: char){
    println!("The value of x is {x}{y}");
}

fn five() -> i32{
   5
}

fn plus_one(num: i32) -> i32 {
    num + 1
}

