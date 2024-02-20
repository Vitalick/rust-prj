use std::{time};
use std::fmt::{Debug};
use async_std::task;

pub(crate) async fn print_start_end(wait_seconds: u64, number: u8){
    let s = format!("TASK #{number}: ");
    println!("{s}Wait for {wait_seconds} s.");
    task::sleep(time::Duration::from_secs(wait_seconds)).await;
    println!("{s}Waited.");
}