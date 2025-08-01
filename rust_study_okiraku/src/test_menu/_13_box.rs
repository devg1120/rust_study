
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {

/*
 Rust で「ヒープ領域」からメモリを取得するときは Box<T> を使う
Box<T> は C++ (C++11) の「スマートポインタ」の動作に近い
Box<T> がスコープから外れたとき、取得したメモリは自動的に解放される
メモリの取得はメソッド new() で行う
Box::new(データ) -> Box<データ型>
格納されているデータは * で取得できる (デリファレンス)
トレイト Drop のメソッド drop() を実装すると、メモリを解放するときに処理を実行することができる
*/

println!("--------------------  box");
{
// 点
struct Point { x: f64, y: f64 }

// Point を廃棄したときにメッセージを表示する
impl Drop for Point {
    fn drop(&mut self) {
        println!("Drop Point!");
    }
}

fn main() {
    let a = Box::new(10);
    println!("{}", *a);    // * でデリファレンス
                           // a でも表示できる
    let b = a;             // move
    // println!("{}", a);  コンパイルエラー
    println!("{}", b);
    {
        let p1 = Box::new(Point {x: 0.0, y: 0.0});
        println!("{}, {}", p1.x, p1.y);  // (*p1).x, (*p1).y と同じ
        let Point {x: c, y: d} = *p1;    // パターンマッチ
        println!("{}, {}", c, d);
    }
    // ここで p1 が廃棄される
    println!("----- end ------")
    // main() が終了したら b が廃棄される
}
main();
}

println!("--------------------  box");
{
#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Box<List>)
}

impl Drop for List {
    fn drop(&mut self) {
        println!("Drop List {:?}", self);
    }
}

impl List {
    // 空リストを返す
    fn new() -> List {
        List::Nil
    }

    // 連結リストの先頭にデータを追加する
    fn cons(self, x: i32) -> List {
        List::Cons(x, Box::new(self))
    }
}

fn main() {
    let a = List::new();
    let b = a.cons(1);       // a は move
    println!("{:?}", b);
    {
        let c = b.cons(2);   // b は move
        println!("{:?}", c);
    }
    // ここで c がすべて廃棄される
    println!("----- end -----");
}
main();
}

}

