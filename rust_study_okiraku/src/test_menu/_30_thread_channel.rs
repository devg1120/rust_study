
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  スレッド");
{
use std::{thread, time};

fn foo(msg: &str, n: u64) {
    let m = time::Duration::from_millis(n);
    for _ in 1 .. 10 {
        println!("{}", msg);
        thread::sleep(m);
    }
}

fn main() {
    let t1 = thread::spawn(|| {
        foo("oops", 500);
    });
    let t2 = thread::spawn(|| {
        foo("piyopiyo", 400);
    });
    println!("{:?}", t1.join().unwrap());
    println!("{:?}", t2.join().unwrap());
}
main();
}

println!("--------------------  スレッド");
{
use std::thread;
use std::time::Instant;

// フィボナッチ数列
fn fibo(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    let s1 = Instant::now();
    println!("{}", fibo(40));
    let e1 = s1.elapsed();
    println!("{}.{:03}秒経過しました。", e1.as_secs(), e1.subsec_nanos() / 1_000_000);

    let s2 = Instant::now();
    let mut buff: Vec<_> = vec![];
    buff.push(thread::spawn(|| fibo(40)));
    buff.push(thread::spawn(|| fibo(40)));
    for x in buff {
        println!("{}", x.join().unwrap());
    }
    let e2 = s2.elapsed();
    println!("{}.{:03}秒経過しました。", e2.as_secs(), e2.subsec_nanos() / 1_000_000);
}
main();
}
println!("--------------------  チャネル");
{
use std::thread;
use std::sync::mpsc;

fn fibo(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for x in vec![40, 39, 38, 37] {
        let tx = tx.clone();
        thread::spawn(move || tx.send(fibo(x)).unwrap());
    }
    for _ in 0 .. 4 {
        println!("{}", rx.recv().unwrap());
    }
}
main();
}
println!("--------------------  Arc と Mutex の簡単な使用例");
/*
スレッド間で immutable なデータを共有したい場合は Arc<T> を使う
Rc をスレッドでも使用できるようにしたものが Arc
*/
{
use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let data = Arc::new(vec![10, 20, 30, 40]);
    for i in 0 .. 4 {
        let data = data.clone();
        thread::spawn(move || {
            println!("{}", data[i]);
        }).join().unwrap();
    }
    let data1 = Arc::new(Mutex::new(vec![1, 2, 3, 4]));
    for i in 0 .. 4 {
        let data1 = data1.clone();
        thread::spawn(move || {
            let mut buff = data1.lock().unwrap();
            buff[i] += 10;
            println!("{}", buff[i]);
        }).join().unwrap();
    }
    println!("{:?}", data1);
}
main();
}
println!("--------------------   RwLock の簡単な使用例 (1)");
{
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(123);
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("{}, {}", *r1, *r2);
        //let mut w1 = lock.write().unwrap();   ブロックされる
        //*w1 = 456;
        //println!("{}", *w1);
    }
    {
        let mut w1 = lock.write().unwrap();
        *w1 = 456;
        println!("{}", *w1);
        //let mut w2 = lock.write().unwrap();   ブロックされる
        //*w2 = 789;
        //println!("{}", *w2);
        //let r3 = lock.read().unwrap();   　   ブロックされる
        //println!("{}", *r3);
    }
}
main();
}


println!("--------------------   RwLock の簡単な使用例 (2)");
{
use std::{thread, time};
use std::sync::{Arc, RwLock};

fn main() {
    let lock = Arc::new(RwLock::new(1));
    let t1;
    let t2;
    {
        let lock = lock.clone();
        t1 = thread::spawn(move || {
            for _ in 0 .. 10 {
                match lock.try_read() {
                    Ok(r) => println!("read data {}", *r),
                    _ => println!("Can't read data")
                }
                thread::sleep(time::Duration::from_millis(500));
            }
        });
    }
    {
        let lock = lock.clone();
        t2 = thread::spawn(move || {
            for _ in 0 .. 10 {
                match lock.try_write() {
                    Ok(mut w) => {
                        *w+= 1;
                        println!("write data {}", *w);
                    }
                    _ => println!("Can't write data")
                }
                thread::sleep(time::Duration::from_millis(500));
            }
        });
    }
    t1.join().unwrap();
    t2.join().unwrap();
}
main();
}

}
