pub fn fn1(){
	let mut v = Vec::new();
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);
	
	let v = vec![1, 2, 3, 4, 5];
	// примет ссылку на значение из вектора
	// если индекс за пределами вектора - вызовет панику
	let third: &i32 = &v[2]; 
	println!("The third element is {third}");

	// так же вызывает значение из вектора по инжексу
	// если индекс за пределами, вернет None, который можно отработать без паники
	let third: Option<&i32> = v.get(29);
	match third {
		Some(third) => println!("The third element is {third}"),
		None => println!("There is no third element.")
	}
	
	let s = vec![1, 2, 3, 4, 5];
	let first = &s[0];
	//s.push(6);
	println!("The first element is {first}");

}

pub fn fn2(){
	println!("");
	let v = vec![100, 32, 57];
    for i in &v {
        println!(">>> {i}");
    }

	let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
		println!(">>> {i}");
    }

    for i in &v {
        println!(">>> {i}");
    }
	
	// для хранения в векторе значений разного типа можно использовать enum
	// использование перечисления вместе с выражением match означает, 
	// что во время компиляции Rust гарантирует, что все возможные случаи будут обработаны
	enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


