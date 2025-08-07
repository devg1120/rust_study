
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
https://blog.ojisan.io/rust-dispatch/
Rust の 動的ディスパッチとか静的ディスパッチとか


動的ディスパッチ
実行時にオブジェクトの型を調べ、対応するメソッドを呼び出す。

静的ディスパッチ
コンパイル時に呼び出すべきメソッドを決める、決まる。

*/


//dynキーワードを使わずに型を決定させる事 → そのために、型パラメーターにtrait境界を指定し、コンパイル時にコンパイラが型を特定できるようにする → 静的ディスパッチになる


println!("--------------------  静的ディスパッチ");
{

trait Foo { 
    fn method(&self) -> String;
}
impl Foo for u8 { 
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Foo for String { 
    fn method(&self) -> String {
        format!("string: {}", *self) 
    } 
}

fn do_something<T: Foo>(x: T) -> String{
    x.method()
}

fn main() {

    let x = 5u8;
    let y = "Hello".to_string();

   let ans1 =  do_something(x);
   let ans2 =  do_something(y);
   println!("{}", ans1);
   println!("{}", ans2);
                               
}
main();
}
println!("--------------------  静的　動的　違い");
{
struct HogeA{}
struct HogeB{}

trait TestTrait{
	fn trait_func(&self);
}
impl TestTrait for HogeA{
	fn trait_func(&self) {
            println!("{}", "  HoheA");
	}
}
impl TestTrait for HogeB{
	fn trait_func(&self) {
            println!("{}", "  HoheB");
	}
}

fn hoge_impl(a:&impl TestTrait){  //静的ディスパッチ  コンパイル時に型が特定
	a.trait_func();
}
//糖衣構文
fn hoge_impl2<T:TestTrait>(a:&T){  //静的ディスパッチ  コンパイル時に型が特定
	a.trait_func();
}

//fn hoge(a:&impl Trait)
//fn hoge<T:Trait>(a:&T)

fn hoge_dyn(a:&dyn TestTrait){    //動的ディスパッチ 実行時に型を特定

	a.trait_func();
}


fn main(){
	let a=HogeA{};
        println!("{}", "static dispatch");
	hoge_impl(&a);
        println!("{}", "static dispatch syntacs syuger");
	hoge_impl2(&a);
        println!("{}", "dynamic dispatch");
	hoge_dyn(&a);
}
main();

}
println!("--------------------   ジェネリックトレイト");
{
// Non-copyable types.
// コピー不可な型
// 訳注: `clone()`メソッドを用いないかぎり、値のコピーではなくムーブが起きる型
struct Empty;
struct Null;

// A trait generic over `T`.
// ジェネリック型 `T`に対するトレイト
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an
    // additional single parameter `T` and does nothing with it.
    // `self`に加えてもう一つジェネリック型を受け取り、
    // 何もしないメソッドのシグネチャを定義
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and
// caller `U`.
// `U`を`self`として、`T`をもう一つの引数として受け取る`DoubleDrop<T>`
// を実装する。`U`,`T`はいずれもジェネリック型
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments,
    // deallocating both.
    // このメソッドは2つの引数の所有権を取り、メモリ上から開放する。
    fn double_drop(self, _: T) {
           println!("{}", "double_drop");
    }
}

fn main() {
    let empty = Empty;
    let null  = Null;

    // Deallocate `empty` and `null`.
    // `empty`と`null`を開放
    empty.double_drop(null);

    //empty;
    //null;
    // ^ TODO: Try uncommenting these lines.
    // ^ TODO: これらの行をアンコメントしてみましょう。
}
main();
}

println!("--------------------   ジェネリックトレイト");
/*
ジェネリックトレイトとは
ジェネリックトレイトは、異なるデータ型で共通の動作を定義するための機能です。

ジェネリックを使用することで、型に依存しない柔軟なコードを書くことができます。
ジェネリックトレイトを定義するにはトレイト定義時に型パラメータを使用します。
*/

{
trait Adder<T> {                   //ジェネリックトレイト
    fn add(&self, value: T) -> T;
}

#[derive(Debug)]
struct Number { value: i32 }
#[derive(Debug)]
struct Str { value: String }

#[derive(Debug)]
struct NumberStr { value: i32 }
trait NumberStrAdder<i32> {                   //ジェネリックトレイト
    fn add(&self, value: i32) -> String;
}


impl Adder<i32> for Number {
    fn add(&self, value: i32) -> i32 {
        self.value + value
    }
}

impl Adder<String> for Str {
    fn add(&self, value: String) -> String {
        //self.value + value
        format!("{}{}", self.value, value)
    }
}

impl NumberStrAdder<i32> for NumberStr {
    fn add(&self, value: i32) -> String {
        format!("{}{}", self.value, value.to_string())
    }
}

fn main() {
   let n1 = Number{ value: 99};
   let ans = n1.add(1);
   println!("{:?} => {}",n1, ans);

   let s1 = Str{ value: "xyz".to_string()};
   let ans = s1.add("ABC".to_string());
   println!("{:?} => {}",s1, ans);

   let ns1 = NumberStr{ value: 99};
   let ans = ns1.add(1);
   println!("{:?} => {}",ns1, ans);

}

main();
}
println!("--------------------  ジェネリクスを使ってみる");
{
fn is_equal(a: i32, b: i32) -> bool {
    if a == b { 
        true
    } else {
        false
    }   
}

//Eqトレイトを実装している型であれば何でも受け付けますよ、という宣言
fn is_equal_generic<T: Eq>(a: T, b: T) -> bool {
    if a == b {
        true
    } else {
        false
    }
}
fn main() {
    println!("{}", is_equal(1, 1)); // true

    println!("{}", is_equal_generic(1, 1)); // true
    println!("{}", is_equal_generic("1", "1")); // true}
}
main();
}
println!("--------------------  トレイトを使ってみる");
/*
トレイトを使うことにより、異なる型に共通の属性を持たせることができる。
*/
{
trait Calculatable {
    // 結果を取り出すことができますよ、という属性。
    // この属性を持つには、その属性を持ちたい型がresultという
    // メソッドを実装している必要がある。
    fn result(self) -> i32;
}

impl Calculatable for i32 {
    fn result(self) -> i32 {
        self // 数値はそのまま返す
    }
}

impl Calculatable for &str {
    fn result(self) -> i32 {
        self.parse().unwrap() // 文字列は変換して返す
    }
}

fn is_equal<T: Calculatable, U: Calculatable>(a: T, b: U) -> bool {
    if a.result() == b.result() {
        true
    } else {
        false
    }
}

fn main() {
    println!("{}", is_equal(1, 1)); // true
    println!("{}", is_equal("1", "1")); // true
    println!("{}", is_equal(1, "1")); // true
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
}

