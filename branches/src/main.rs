use std::io;

fn main() {
    println!(">>> ");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("ERROR1");
    let num: i32 = input.trim().parse().expect("ERROR2");
    if num < 5 {
        println!("true");
    } else if num % 2 == 0 {
        println!("WOu");
    } else { 
        println!("false");
    }

    let p: String = if num == 11 { "Snake eyes".to_string()} else {"no".to_string()};
    println!("{p}");


    fn2();
    fn3();
    fn4();
    fn5();
}

fn fn2() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}


fn fn3() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remeining = 10;
   
        loop {
             println!("remeining = {remeining}");
             if remeining == 9 {
                 break;
             }
             if count == 2 {
                 break 'counting_up;
             }
             remeining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}


fn fn4() {
    let mut number = 3;
 
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


fn fn5() {
    let a = [10,20,30,40,50];
    let mut index = 0;
    
    while index < 5{
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("element = {element}");
    }

    for num in (1..=3).rev() {
        println!("{num}!");
    }
    println!("Doen!");

}
