mod test_async;

use std::cmp::Ordering;
use std::{io};
use futures::future::join_all;
use rand::{Rng};


#[tokio::main]
async fn main() {
    println!("Загадайте число!");
    let mut local_rng = rand::thread_rng();

    let mut tasks = Vec::new();

    for i in 1..=10 {
        let task = test_async::print_start_end(local_rng.gen_range(1..=10), i);
        tasks.push(task);
    }

    // Wait all tasks like Promise.all() in Javascript
    join_all(tasks).await;
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