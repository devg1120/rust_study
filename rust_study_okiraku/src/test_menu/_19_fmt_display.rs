
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {
/*
format! は書式文字列に従ってデータを整形し、その結果を文字列にして返す
print!, println! マクロは書式文字列に従ってデータを整形して標準出力に書き込む
write!, writeln! マクロは第 1 引数に整形した結果を書き込む
第 1 引数はトレイト std::io::Write を実装しているデータ型であること
write!, writeln! は Result 型を返す
書式文字列の {} は引数を文字列に変換する
{n} とすると n 番目の引数を変換する
{:変換指示子}
{:b}, 2 進数表示
{:o}, 8 進数表示
{:x}, {:X}, 16 進数表示 (英小文字, 英大文字)
{:e}, {:E}, 指数表示 (英小文字, 英大文字)
{:?}, Debug 表示
: と変換指示子の間に桁数などを指定することができる
{:n}, n 桁表示
{:<n}, 左詰め
{:>n}, 右詰め
{:^n}, 中央
{:0n}, n 桁表示で空きを 0 で埋める
{:.n}, 小数点以下の桁数を指定する
*/

println!("--------------------  format! ");
{
use std::io::{self, Write};

fn main () {
    let mut buff = Vec::new();
    write!(&mut buff, "hello, world").unwrap();
    write!(&mut buff, "{}, {}, {}", 1, 1.2345, "foo bar baz").unwrap();
    io::stdout().write(&buff).unwrap();
    println!("");
}
main();
}

println!("--------------------  Display トレイト");
/*
std::fmt::Display トレイトを実装すると、書式文字列の {} でデータを変換できる
pub trait Display {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error>;
}
*/
{
use std::fmt;

struct Point {
    x: i32, y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 10, y: 20 };
    println!("{}, {}", a, b);
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

}

