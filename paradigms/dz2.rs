fn main(){
    let number: i32 = input().trim().parse().expect("err_type_String_to_i32");
    pifagor_table(number);

    multiplication_table(number);
}

fn pifagor_table(num: i32){
    print!("   ");
    for i in 1..=num {
        print!(" {:4}", i);
    }
    println!("");

    for j in 1..=num {
        print!(" {:2}", j);
        for k in 1..=num{
            print!(" {:4}", j * k)    
        }
        println!("");
    }
    println!("\n");
}

fn multiplication_table(num: i32){
    for i in 1..=num {
        from_1_to_9(i);
        println!("");
    }
}

fn from_1_to_9(num: i32){
    for i in 1..=9 {
        print!("{:2}  * {:2}  = {:3}  ", i, num, i * num);
    }
}

fn input() -> String {
    use std::io;
    println!("Enter number: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("err_input");
    input
}
