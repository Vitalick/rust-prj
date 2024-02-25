use std::io;
use std::str::FromStr;
use num_traits::Num;

fn scan_data() -> String {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Не удалось распознать ввод");
    return n;
}

fn scan_array<T: Num + FromStr<Err=std::num::ParseIntError>>() -> Vec<T> {
    let data = scan_data();
    let arr = data.trim().split(" ");
    let mut vec: Vec<T> = Vec::new();
    for v in arr {
        vec.push(v.parse().expect("Вы вписали не числа!"));
    }
    return vec;
}

fn main() {
    println!("Введите массив через пробел:");
    let mut vec: Vec<i64> = scan_array();
    println!("Введённый массив: {:?}", vec);
    vec.retain(|&x| x % 2 != 0);
    println!("Обработанный массив: {:?}", vec);
}