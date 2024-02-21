use std::{time};

use async_std::task;
use futures::future::join_all;
use rand::Rng;


pub async fn test_async_printing<T: Rng>(local_rng: &mut T, amount: u64) {
    let mut tasks = Vec::new();

    for i in 1..=amount {
        let task = print_start_end(local_rng.gen_range(1..=10), i);
        tasks.push(task);
    }

    join_all(tasks).await;
}
async fn print_start_end(wait_seconds: u64, number: u64){
    let s = format!("TASK #{number}: ");
    println!("{s}Wait for {wait_seconds} s.");
    task::sleep(time::Duration::from_secs(wait_seconds)).await;
    println!("{s}Waited.");
}