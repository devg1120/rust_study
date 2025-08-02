
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

/*
#[derive(Debug)]
println!("{:#?}", foo);
*/
pub fn main() {

/*
RustのTokioで非同期とグリーンスレッドを理解する
https://zenn.dev/tfutada/articles/5e87d6e7131e8e
*/

println!("--------------------  ");
{
use std::time::Duration;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    let cpus = num_cpus::get();
    println!("logical cores: {}", cpus);

    tokio::spawn(async move {
        println!("task 1 started...");
        std::thread::sleep(Duration::from_secs(3)); // 3秒寝る(同期スリープ)
        println!("task 1 woke up!");
    });
    // この時点でtask1はすでに走っている。
    
    tokio::spawn(async move {
        println!("task 2 started...");
        std::thread::sleep(Duration::from_secs(1));
        println!("task 2 woke up!");
    });
    // この時点でtask2は走らない。空きスレッドがないから。
    std::thread::sleep(Duration::from_secs(5)); // 3秒後にtask2は走る

    println!("Done");
}
main();
}

println!("--------------------  ");
/*
 タスクAを１つ、タスクBを1000個生成します。タスクAは10秒同期スリープします。
*/
{
use std::time::Duration;
use futures::{stream::FuturesUnordered, StreamExt};

// #[tokio::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    //console_subscriber::init();

    let mut tasks = FuturesUnordered::new();

    let h1 = tokio::spawn(async move {
        std::thread::sleep(Duration::from_secs(10)); // ここでつまるよ！
        println!("0 woke up!");
    });
    tasks.push(h1);
    println!("0 started...");

    for i in 1..1000 {
        let h2 = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("{i} woke up!");
        });
        tasks.push(h2);
        println!("{i} ...");
    }

    // join_allは遅いよ。https://github.com/tokio-rs/tokio/issues/2401
    while let Some(item) = tasks.next().await {
        let () = item.unwrap();
    }
    println!("Done");
}
main();
}
println!("--------------------  チャネル");
{
use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main()]
async fn main() {
    // チャネル (胃)
    let (tx1, mut rx1) = mpsc::channel(2);

    // 送信側タスク (口)
    let handle1 = tokio::spawn(async move {
        let mut count: i32 = 0;

        while count < 5 {
            if let Err(e) = tx1.send(count).await { // 食べる
                eprintln!("{}", e);
                break;
            }
            println!("sent {}", count);
            count += 1;
        }
    });
    // この時点で食事はすでに進んでいます。

    // 受信側タスク (小腸)
    let handle2 = tokio::spawn(async move {
        while let Some(msg) = rx1.recv().await { // 消化
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("get {}", msg);
        }
    });

    let (_r1, _r2) = (handle1.await.unwrap(), handle2.await.unwrap());
}
main();
}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  async tokio");
{
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("メイン処理開始");

    // 複数の非同期タスクを生成し、並行実行
    let handle_a = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("タスクA完了");
    });

    let handle_b = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        println!("タスクB完了");
    });

    let handle_c = tokio::spawn(async {
        sleep(Duration::from_millis(150)).await;
        println!("タスクC完了");
    });

    // tokio::join! を使って全てのタスクが完了するのを待つ
    // これにより、メイン関数がタスクの完了を待たずに終了してしまうのを防ぎます。
    tokio::join!(handle_a, handle_b, handle_c);

    println!("メイン処理終了");
}
main();
}
println!("--------------------  tokio::task::spawn_blocking の使用");
/*
これは、Tokio 内で CPU 集約型のタスクを実行する際の第一選択肢です。spawn_blocking 関数は、指定されたクロージャを専用のスレッドプールに移して実行します。このスレッドプールは Tokio ランタイムとは分離されているため、CPU 集約型のタスクが Tokio のランタイムスレッドをブロックすることはありません。その結果、他の I/O 集約型のタスクにも悪影響を与えずに実行できます。
*/
{
use tokio::task;

#[tokio::main]
async fn main() {
    // 非同期 I/O 処理の開始
    println!("I/O 処理開始");
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    println!("I/O 処理完了");

    // CPU 集約型タスクの実行
    let result = task::spawn_blocking(move || {
        println!("CPU 集約型タスク開始");
        // 重い計算処理
        let mut sum = 0u64;
        for i in 0..100000000 {
            sum += i;
        }
        println!("CPU 集約型タスク完了");
        sum
    }).await.unwrap();

    println!("計算結果：{}", result);
}
main();
}
println!("--------------------  独立スレッドプールの使用");
/*
もう一つの方法は、CPU 集約型のタスクを独立したスレッドプールで実行することです。これには std::thread や rayon のようなライブラリを用いることができます。非同期ランタイムである Tokio との間でデータをやり取りするためには、std::sync::mpsc や tokio::sync::mpsc などのチャネルを使用します。
*/
{
use std::thread;
use std::sync::mpsc;
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();
    let (tx, rx) = mpsc::channel();

    rt.block_on(async {
        // 非同期 I/O 処理の開始
        println!("I/O 処理開始");
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        println!("I/O 処理完了");

        // CPU 集約型タスクをスレッドプールに送信
        let tx_clone = tx.clone();
        thread::spawn(move || {
            println!("CPU 集約型タスク開始");
            // 重い計算処理
            let mut sum = 0u64;
            for i in 0..100000000 {
                sum += i;
            }
            println!("CPU 集約型タスク完了");
            tx_clone.send(sum).unwrap();
        });

        // チャネルから結果を受信
        let result = rx.recv().unwrap();
        println!("計算結果：{}", result);
    });
}
main();
}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
}

