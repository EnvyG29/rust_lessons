use std::collections::HashMap;  

pub fn task(){
	let input: String = input(String::from(">>> "));
	let v: Vec<i32> = input_to_vector(input);
	println!("{:?}", v);

	let mean: f32 = mean(&v);
	println!("mean = {}", mean);
	
	let median: i32 = median(&v);
	println!("median = {}", median);

	let mode: i32 = mode(&v);
	println!("mode = {}", mode);
}

// функция ввода данных через терминал
fn input(s: String) -> String {
	use std::io; // импорт из стандартной библиотеки модуль для ввода
	println!("{} ", s); 
	let mut input: String = String::new();
	io::stdin() // создает экземпляр типа Stdin, который предоставляет поток ввода из стандартного потока ввода (обычно клавиатуры).

		.read_line(&mut input) //  читает строку из ввода и сохраняет ее в переменной input. &mut input означает, что мы передаем изменяемую ссылку на переменную input, что позволяет методу изменять ее значение.

		.expect("err input"); // обрабатывает возможную ошибку процесса чтения строки с ввода. Если произойдет ошибка, программа завершится с сообщением "err input".
	input
}

fn input_to_vector(nums: String) -> Vec<i32>{
	let mut v: Vec<i32> = Vec::new(); 
	for num in nums.split_whitespace(){ // проходим по итератору который разбил строку по пробелам
		let n: i32 = num.parse().expect("only nums!!"); // приводим строковый срез к типу i32
		v.push(n); // добавляем элемент в вектор
	}
	v
}

fn mean(v: &Vec<i32>) -> f32{
	let sum: i32 = v.iter() // создаем итератор 
		.sum(); // сумируем все значения в итераторе
	let mean: f32 = sum as f32 / v.len() as f32;
	mean
}

fn median(v: &Vec<i32>) -> i32{
	let mut clone_v: Vec<i32> = v.clone(); // полностью копируем из ссылки в новый вектор
	clone_v.sort(); // сортируем верктор
	let median = clone_v[v.len() / 2];
	median
}

fn mode(v: &Vec<i32>) -> i32{
	let mut map: HashMap <i32, i32> = HashMap::new();
	let mut max_count: i32 = 0;
	let mut mode: i32 = 0;

	for &num in v{
		let count = map.entry(num) // проверяем есть ли ключ в HashMap
			.or_insert(0); // если ключа нет, добавить со значением 0
		*count += 1; // разыменовывание переменной и увеличение его значения

		if *count > max_count { 
			max_count = *count;
			mode = num;
		}
	}
	mode
}
