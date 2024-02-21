mod test_async;

use std::cmp::Ordering;
use std::{io};
use std::fmt::Debug;
use rand::{Rng};
use rand::distributions::uniform::SampleRange;

#[tokio::main]
async fn main() {
    let mut local_rng = rand::thread_rng();
    let mut array: [u64; 5] = [0;5];
    for v in &mut array {
        *v = local_rng.gen_range(1..=10)
    }
    test_async::test_async_printing_arr(array.as_slice()).await;
    // test_async::test_async_printing(&mut local_rng, 10).await;

    let secret_number = local_rng.gen_range(1..=100);

    println!("Загадайте число!");
    loop {
        println!("Пожалуйста введите число от 1 до 100.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка чтения строки");


        let guess: u32 = match guess.trim().parse() {
         Ok(num) => num,
            Err(_) => {
                println!("Вы вписали не число!");
                continue;
            }
        };
        println!("Вы загадали: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком низкое число"),
            Ordering::Greater => println!("Слишком высокое число"),
            Ordering::Equal => {
                println!("Вы отгадали!");
                return;
            }
        }
    }
}