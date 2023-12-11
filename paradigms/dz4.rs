fn main() {
    let vec1: Vec<f64> = vec![2.0, 4.0, 6.0, 8.0];
    let vec2: Vec<f64> = vec![2.0, 4.0, 10.0, 12.0];
    let corr = corr_pirson(&vec1, &vec2);
    println!("{}", corr);
}

fn corr_pirson(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    let mean_v1: f64 = mean_vec(v1);
    let mean_v2: f64 = mean_vec(v2);

    let numerator: f64 = v1
        .iter() // создает итератор
        .zip(v2) // объединяет два итератера в один создавая пары элементов.
        // если тип реализует трейт intoiter, то явно преобразовывть в итератор не нужно
        .map(|(x, y)| (x - mean_v1) * (y - mean_v2))
        .sum();

    let deniminator_v1: f64 = v1
        .iter()
        .map(|x| (x - mean_v1).powi(2)) // возводит целочисленную степень
        .sum::<f64>() // в какой тип преоразовать данные при сумировании
        .sqrt();

    let deniminator_v2: f64 = v2.iter().map(|y| (y - mean_v2).powi(2)).sum::<f64>().sqrt();

    numerator / (deniminator_v1 * deniminator_v2)
}

fn mean_vec(v: &Vec<f64>) -> f64 {
    let sum: f64 = v.iter().sum();
    sum / (v.len() as f64) // принудительно меняет тип данных
}
