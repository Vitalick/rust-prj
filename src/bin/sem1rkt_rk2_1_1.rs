use std::io;
use std::str::FromStr;
use num_traits::Num;

fn scan_number<T: Num + FromStr<Err=std::num::ParseIntError>>() -> T {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Не удалось распознать ввод");
    let n: T = n.trim().parse().expect("Вы вписали не число!");
    return n;
}

fn input() -> Vec<i64> {
    println!("Введите размер массива:");
    let n: usize = scan_number();
    let mut vec = Vec::new();
    vec.resize(n, 0);
    println!("Введите массив через разрыв строки:");
    for i in 0..n {
        vec[i] = scan_number();
    }
    return vec;
}

fn main() {
    let mut vec = input();
    println!("Введённый массив: {:?}", vec);
    vec.retain(|&x| x % 2 != 0);
    println!("Обработанный массив: {:?}", vec);
}