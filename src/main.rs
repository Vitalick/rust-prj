mod test_async;

use std::cmp::Ordering;
use std::{io};
use rand::{Rng};


#[tokio::main]
async fn main() {
    println!("Загадайте число!");
    let mut local_rng = rand::thread_rng();

    test_async::test_async_printing(&mut local_rng, 10).await;

    let secret_number = local_rng.gen_range(1..=100);

    loop {
        println!("Пожалуйста введите число от 1 до 100.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка чтения строки");

        println!("Вы загадали: {guess}");

        let guess: u32 = guess.trim().parse().expect("Вы вписали не число!");

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