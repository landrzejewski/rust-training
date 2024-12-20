use std::thread;
use std::time::Duration;

use reqwest::Response;
use tokio::time;

async fn task() -> i32 {
    println!("Calculating...{:?}", thread::current());
    time::sleep(Duration::from_secs(2)).await;
    println!("After calculating...");
    4
}

async fn get_time() -> reqwest::Result<Response> {
    println!("Sending request... {:?}", thread::current());
    let api = "http://worldtimeapi.org/api/timezone/Europe/Warsaw";
    let result = reqwest::get(api).await;
    println!("After request...");
    result
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
pub async fn run() {
    let result = task().await;
    println!("Value: {result}");
    println!("Before end");

    // let result = tokio::spawn(get_time());

    // let result = spawn(task());
    println!("Before sleep {:?}", thread::current());
    time::sleep(Duration::from_secs(5)).await;
    println!("After sleep");

    // println!("{:?}", result.await);
}
