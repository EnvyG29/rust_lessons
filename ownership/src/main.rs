fn main() {
    fn1();
}

fn fn1(){
    let s1 = String::from("python");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
