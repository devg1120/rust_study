
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


/*
 *  【Rust】クロージャと関数ポインタ
 *   https://zenn.dev/nuskey/articles/rust-closure-function-pointer
 */

pub fn main() {


println!("--------------------  closure");
{
let double = |a| a * 2;

let sub = |a, b| {
    a - b
};
println!("{}", double(2)); // 4
println!("{}", sub(1, 2)); // -1
}

println!("-------------------- 外部変数のキャプチャ");
{
let x = "hello".to_string();

let f = || {
    // クロージャ内部で外部のローカル変数xを参照する
    println!("{}", x);
};

f(); // hello!
}
println!("--------------------  mut closure");
{
let mut x = 10;

// クロージャもmut変数にする
let mut f = || {
    // 内部でxを書き換え
    x = 20;
};

f();
println!("{}", x); // 20
                  
}
println!("--------------------  クロージャと所有権");
/*
moveキーワードを用いて明示的に変数の所有権を移動させる
*/
{
fn create_closure() -> impl Fn() -> () {
    let x = "hello".to_string();
    
    // xの所有権をクロージャに移動させる
    let f = move || {
        println!("{}", x);
    };
    
    // xの所有権が移動されているのでOK
    return f;
}
let x = create_closure();
x();

}
println!("--------------------  クロージャを引数として受け取る");
{
// 受け取ったクロージャを呼び出し、その結果を返すだけの関数
fn call_closure<T, U>(f: T)  -> U
    where T: Fn() -> U {
    f()
}

// 引数の型も指定できる
fn call_closure_with_args<T, U>(f: T, args: U) -> U
    where T: Fn(U) -> U{
    f(args)
}
fn call_closure_with_args2<T, U>(f: T, arg1: U, arg2: U) -> U
    where T: Fn(U,U) -> U{
    f(arg1, arg2)
}

let get = || {
    999
};

let sub = |a:i32| {
     100 - a
};

let add = |a:i32, b:i32| {
     a + b
};

let ans = call_closure(get);
println!("{}", ans);

let ans2 = call_closure_with_args(sub, 3);
println!("{}", ans2);

let ans3 = call_closure_with_args2(add, 9998, 2);
println!("{}", ans3);

}
println!("--------------------  クロージャを返す");
/*
クロージャ用トレイト

  FnOnce : 一度だけの呼び出しが可能なクロージャを表すトレイト
  FnMut  : 内部で何らかの変更を行うクロージャに対して実装されるトレイト
  Fn     : 内部で変数をキャプチャしないか、不変な参照を借用するクロージャに対して実装されるトレイト

*/
{
fn create_mut_closure() -> impl FnMut() -> () {
    let x = "hello".to_string();
    let mut count = 0;
    let f = move || {
        println!("{}", x);
        count = count + 1;
        println!("{}", count);
    };
    
    return f;
}
let mut x = create_mut_closure();
x();
x();
x();

}
println!("-------------------- クロージャを返す 1/N ");
{
fn create_is_zero(x: isize) -> Box<dyn Fn()> {
    if x == 0 {
        Box::new(|| { 
            println!("x is 0");
        })
    } else {
        Box::new(move || { 
            println!("x is not 0. x is {}", x);
        })
    }
}
let x1 = create_is_zero(0);
let x2 = create_is_zero(8);
x1();
x2();
}
println!("--------------------  関数ポインタ");
/*
関数ポインタはfn(T, U) -> Vのような型で表され
*/
{
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 関数ポインタを受け取り、(1, 2)を渡して呼び出した結果を返す
fn call_fn_ptr(f: fn(i32, i32) -> i32) -> i32 {
    f(1, 2)
}

fn main() {
    // addを関数ポインタとして渡す
    let x = call_fn_ptr(add);

    println!("{}", x); // 3
}
main();
}
println!("--------------------  関数ポインタ → クロージャ");
/*
Rustの関数ポインタはFnOnce、FnMut、Fnトレイトを全て実装しています。
そのためクロージャを受け取る関数に関数ポインタを渡すことも可能です。
*/
{
fn foo() {
    println!("foo");
}

fn call_closure<T: Fn()>(f: T) {
    f();
}
fn main() {
    // 関数ポインタをcall_closureに渡す
    call_closure(foo); // foo
}
main();
}
}

