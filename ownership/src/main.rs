fn main() {
    fn1();
    println!("");
    
    fn2();
    fn4();
    fn3();
    fn5();
    fn6();
}

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
fn fn1(){
    let s1 = String::from("python");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let s = "word";
    print_type(s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}



fn fn2() {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("err_input");
    input = input.trim().to_owned();
    let i = &input;
    
    println!(">>> {input} ...{i}");
    println!(">>> {input} ...{i}{i}");


    let p = input;
    println!(">>> {p} ...");
   
}

fn fn4() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,

    println!(">>> {x} ...");
                                   // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn fn3(){
    let mut s = String::from("java");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem

    println!("{}", r3)
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn fn5() {
    let mut s = String::from("rust and");

    s.push_str(" python");

    let word = first_word(&s);

   // s.clear(); // error!

    println!("{s}\nthe first word is: {}", word);
}

fn fn6() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);


    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
