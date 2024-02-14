use std::collections::HashMap;
use std::future::Future;
use std::thread;
use std::time::Duration;

use futures::future;
use futures::future::Ready;
use reqwest::Response;
use tokio::task;
use tokio::task::JoinHandle;

async fn task() -> i32 {
    println!("Calculating...{:?}", thread::current());
    tokio::time::sleep(Duration::from_secs(2)).await;
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
async fn main() {
    /*let result = task().await;
    println!("Value: {result}");
    println!("Before end");*/

    //let result = tokio::spawn(get_time());

    let result = tokio::spawn(task());

    println!("Before sleep {:?}", thread::current());
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("After sleep");

    // println!("{:?}", result.await.expect("Erro").unwrap().text().await);

    println!("{:?}", result.await);

    /*
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .spawn(async {
        });
    */
}
