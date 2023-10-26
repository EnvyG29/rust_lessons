fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value if x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS} in three hours in seconds");

    let x = x * x;
    
    println!("The value if x is: {}", x);
    
    {
	let x = x - 20;
	println!("The value if x is: {}", x);
    }

    println!("The value if x is: {}", x);

    let x = "Python";

    println!("The value if x is: {}", x);

    let x = x.len();

    println!("{x} litter in Python");

    let tup: (i32, String, bool) = (1024, "Python".to_string(), true);

    let (a, b, c) = tup;

    let t: (f32, char) = (3.14, 'R');

    let d = t.1;

    println!("{a}|  {}2,  {}3", c, d);

    let q = [1, 2, 3, 4, 5];

    println!("{}", q[2]);

    let w: [i32; 5] = [0, 1, 2, 3, 4];

    let e = [3; 5];

    println!("{} {} {} {} {}", e[0], e[1], e[4], w[0], w[1]);

}
