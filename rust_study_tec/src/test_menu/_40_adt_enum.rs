
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


println!("--------------------  ");
{
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("{:?}", IpAddrKind::V4); // V4
    println!("{:?}", IpAddrKind::V6); // V6
}
main();
}

println!("--------------------  Enumは個別に型を持てる");
{
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("{:?}", IpAddrKind::V4(127, 0, 0, 1)); // V4(127, 0, 0, 1)
    println!("{:?}", IpAddrKind::V6(String::from("::1"))); // V6("::1")
}
main();
}
println!("--------------------  Enumはメソッドを定義できる");
{
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// メソッドを定義
impl IpAddrKind {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    IpAddrKind::V4(127, 0, 0, 1).call(); // V4(127, 0, 0, 1)
    IpAddrKind::V6(String::from("::1")).call(); // V6("::1")
}
main();
}
println!("--------------------  match");
{
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let color = Color::Red;
    let color_jp = color_to_jp(color);

    println!("{}", color_jp); // あか
}

fn color_to_jp(color: Color) -> String {
    match color {
        Color::Red => String::from("あか"),
        Color::Blue => String::from("あお"),
        Color::Green => String::from("みどり"),
    }
}
main();
}
println!("--------------------  Enumに個別の型がある場合");
{
#[allow(dead_code)]
enum Color {
    Red(String),
    Blue,
    Green,
}

fn main() {
    let color = Color::Red(String::from("#ff0000"));
    let color_jp = color_to_jp(color);

    println!("{}", color_jp); // あか
}

fn color_to_jp(color: Color) -> String {
    match color {
        Color::Red(color_code) => {
            println!("{}", color_code); // #ff0000
            String::from("あか")
        }
        Color::Blue => String::from("あお"),
        Color::Green => String::from("みどり"),
    }
}
main();
}
println!("--------------------  Optionを使いこなそう");
/*
enum Option<T> {
    Some(T),
    None,
}
*/
{
fn main() {
    let some_value = Some(String::from("Hello")); // Someは値がある
    let none_value: Option<String> = None; // Noneは値が無い

    println!("{}", response(some_value)); // Hello World
    println!("{}", response(none_value)); // No value
}

fn response(value: Option<String>) -> String {
    match value {
        Some(value) => {
            value + " World"
        }
        None => String::from("No value"),
    }
}
main();
}
println!("-------------------- 代数的データ型 ");
/*
代数的データ型（Algebraic Data Type, ADT）とは、

複数の型を組み合わせることで定義されるデータ型のことです
関数型プログラミングや型システムにおいて、データの構造を明確に表現するために用いられます

代数的データ型は、直和型と直積型の2つの基本的な概念から構成されます

直積型 (Product Type):

 複数の型を組み合わせて、それら全ての値を保持する型です。
 例えば、{名前: 文字列, 年齢: 整数} のような構造は、文字列型と整数型を組み合わせた直積型の一例です。

直和型 (Sum Type):

 複数の型の中から、いずれか一つの値を保持する型です。
 例えば、結果 = 成功(値: 整数) | 失敗(エラーメッセージ: 文字列) のような型は
 成功時の整数値か、失敗時のエラーメッセージのいずれかを持つ直和型です。


代数的データ型のメリット:

 * 型安全なプログラミング:
    コンパイル時に型チェックを行うことで、実行時のエラーを減らし、
    より安全なコードを記述できます。Qiitaによると

 * データの構造を明確に表現:
     データの種類や構造を明確に定義できるため、コードの可読性や保守性が向上します
 * パターンマッチングとの相性:
     パターンマッチングと組み合わせることで、様々なケースに対応した処理を簡潔に記述できます。


代数的データ型  = 直和型 x 直積型 

    Ex =  T1  A  B   |
       :  T2         |  和 (１つ)
       :  T3  C      |
        
        ----> 積（組み合わせ）
*/
{
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit");
        },
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        },
        Message::Write(text) => {
            println!("Text message: {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change color to Red: {}, Green: {}, Blue: {}", r, g, b);
        },
    }
}
fn main() {

    process_message(Message::Move {x:100,y:200});
    process_message(Message::Write(String::from("abc-xyz 123456789")));
    process_message(Message::ChangeColor( 1,2,3));
    process_message(Message::Quit);

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

