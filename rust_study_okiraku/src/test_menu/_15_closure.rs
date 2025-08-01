
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  closure");
{
    let a = 10;
    let add_a = |x| x + a;      // 変数 a を immutable で借用
    println!("{}", add_a(20));
    let mut b = 0;
    {
        let mut inc_b = || b += 1;   // 変数 b を mutable で借用
        inc_b();
        inc_b();
        inc_b();
        inc_b();
        inc_b();
        // println!("{}", b);   コンパイルエラー
    }
    // mutable な借用は一つしか使用できない
    // inc_b が廃棄されると b にアクセスできる
    println!("{}", b);

    let mut c = 0;
    {
        // move クロージャ (所有権をクロージャに移動する)
        let mut add_c = move |x| {c += x; c};
        // 数値の場合、クロージャ内に値がコピーされ、
        // その値が書き換えられる
        println!("{}", add_c(100));
        println!("{}", add_c(200));
    }
    println!("{}", c);    // c の値は 0 のまま
}

println!("--------------------  クロージャを受け取る高階関数 ");
{
fn apply1<F, V>(func: F, x: V) -> V where F: Fn(V) -> V {
    func(x)
}

fn apply11<V>(func: &dyn Fn(V) -> V, x: V) -> V {
    func(x)
}

fn apply2<F, V>(func: F, x: V, y: V) -> V where F: Fn(V, V) -> V {
    func(x, y)
}

fn apply22<V>(func: &dyn Fn(V, V) -> V, x: V, y: V) -> V {
    func(x, y)
}

fn main() {
    fn square(x: f64) -> f64 { x * x }
    println!("{}", apply1(|x| x * x, 123));
    println!("{}", apply11(&|x| x * x, 123));
    println!("{}", apply1(square, 1.111));
    println!("{}", apply11(&square, 1.111));

    fn add(x: f64, y: f64) -> f64 { x + y }
    println!("{}", apply2(|x, y| x + y, 1, 2));
    println!("{}", apply22(&|x, y| x + y, 1, 2));
    println!("{}", apply2(add, 1.1, 2.2));
    println!("{}", apply22(&add, 1.1, 2.2));
}
main();
}
println!("--------------------  closure  簡単な高階関数");
{
// マッピング
fn map<F, T, U>(func: F, xs: &Vec<T>) -> Vec<U>
    where F: Fn(T) -> U, T: Copy, U: Copy {
    let mut ys: Vec<U> = vec![];
    for x in xs {
        ys.push(func(*x));
    }
    ys
}

// フィルター
fn filter<F, T>(pred: F, xs: &Vec<T>) -> Vec<T>
    where F: Fn(T) -> bool, T: Copy + PartialEq {
    let mut ys: Vec<T> = vec![];
    for x in xs {
        if pred(*x) { ys.push(*x); }
    }
    ys
}

// 畳み込み
fn reduce<F, T, U>(func: F, a: U, xs: &Vec<T>) -> U
    where F: Fn(U, T) -> U, T: Copy, U: Copy {
    let mut acc = a;
    for x in xs {
        acc = func(acc, *x);
    }
    acc
}

fn main() {
    let xs = vec![1,2,3,4,5,6,7,8];
    println!("{:?}", map(|x| x * x, &xs));
    println!("{:?}", map(|x| x as f64 * 1.1, &xs));
    println!("{:?}", filter(|x| x % 2 == 0, &xs));
    println!("{}", reduce(|a, x| a + x, 0, &xs));
}
main();
}
println!("--------------------  closure クロージャを返す");
{

// x を加算するクロージャを返す
fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

// 簡単なジェネレータ 
// (Rust にはイテレータがあるので、このような使い方はあまりしないと思う)
fn make_counter(init: i32) -> Box<dyn FnMut() -> i32> {
    let mut c = init - 1;
    Box::new(move || { c += 1; c})
}

fn main() {
    let add10 = make_adder(10);   // 10 を加算する関数になる
    let add20 = make_adder(20);   // 20 を加算する関数になる
    println!("{}", add10(1));
    println!("{}", add20(2));

    let mut cnt = make_counter(1);   // 1, 2, 3, ... を返す
    for _ in 0..10 {
        println!("{}", cnt());
    }
}
main();
}
}

