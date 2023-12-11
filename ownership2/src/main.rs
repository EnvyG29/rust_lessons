fn main() {
    fn1();

    for i in 1..=15 {
        println!("{}", fn2(i));
    }

    let v: Vec<&str> = vec!["Rust", "Python", "SQL"];
    for i in v.iter() {
        println!("{}", fn3(i));
    }

    let word: String = "ABCDEFG".to_string();
    println!("{}", fn4(word));
    // выпадет ошибка, так как word уже было использовано и потеряло владение над значением
    //println!("{}", fn4(word));
}

fn fn1() {
    let words: String = "Rust Python".to_string();

    let bytes = words.as_bytes();
    for (f, &s) in bytes.iter().enumerate() {
        print!("{:2}--{}     //     ", f, &s);
        println!("{:2}--{}", f, &s); // выведет индекс ячейки и битовый номер значения
    }

    for (f, s) in words.chars().enumerate() {
        print!("{:2}--{}     //     ", f, &s);
        println!("{:2}--'{}'", f, &s);
    }
}

fn fn2(num: u32) -> String {
    print!("{} | ", num);
    format!("{}", num)
}

fn fn3(word: &str) -> &str {
    let l: usize = word.len();
    if l < 5 {
        &word[..l]
    } else {
        &word[5..]
    }
}

fn fn4(w: String) -> String {
    let t: String = format!("{}", w.len());
    t
}
