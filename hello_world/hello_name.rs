fn main() {
    let res: String = hello("Rust");
    println!("{}\n{}\n{}", res, fn2("Python"), fn3(""));
}

fn hello(name: &str) -> String {
    let name = if name.is_empty() {
        "World".to_string()
    } else {
        let first_char: String = name.chars().next().unwrap().to_uppercase().to_string();
        let rest: String = name.chars().skip(1).collect::<String>().to_lowercase();
        format!("{}{}", first_char, rest)
    };
    format!("Hello, {}!", name)
}

fn fn2(name: &str) -> String {
    if name.is_empty() {
        return String::from("Hello, World!");
    }
    format!(
        "Hello, {}{}!",
        name[..1].to_uppercase(),
        name[1..].to_lowercase()
    )
}

fn fn3(name: &str) -> String {
    format!(
        "Hello, {}!",
        match name {
            "" => String::from("World"),
            _ => {
                let (f, s): (&str, &str) = name.split_at(1);
                format!("{}{}", f.to_uppercase(), s.to_lowercase())
            }
        }
    )
}
