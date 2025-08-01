
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  generic ");
{
}

println!("--------------------  generic 1");
{
fn find_if<T>(pred: fn(&T) -> bool, table: &[T]) -> Option<&T> {
    for x in table {
        if pred(x) { return Some(x); }
    }
    None
}

fn main() {
    fn evenp(x: &i32) -> bool { x % 2 == 0 }
    fn oddp(x: &i32) -> bool { x % 2 != 0 }
    let a = [2, 4, 6, 8, 10];
    match find_if(evenp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    match find_if(oddp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
}
main();
}

println!("--------------------  generic 2");
{
// 組
#[derive(Debug, PartialEq)]
struct Pair<T, U> {
    fst: T, snd: U
}

impl <T, U> Pair<T, U> {
    fn new(a: T, b: U) -> Pair<T, U> {
        Pair { fst: a, snd: b }
    }
}

fn main() {
    let p1 = Pair::new(1, 2.0);
    let p2 = Pair::new(1, 3.0);
    let p3 = Pair::new("foo", 100);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{}", p1 == p2);
    println!("{}", p3 == p3);
}
main();
}

/*
ジェネリクスの型パラメータには「境界」を設定することができる
Rust では「トレイト境界」とか「ジェネリック境界」と呼ばれているようだ
<T: U> とした場合、T はトレイト U を実装しているデータ型に制限される
複数のトレイトを境界に指定したい場合は <T: U + V> のように + でつなげる
境界は where 句 (where T: U, ...) を使って設定することもできる
where 句は本体 {...} の直前に挿入する
*/

println!("--------------------  generic 型パラメータの境界");
{

// 距離を求める
trait Distance {
    fn distance(&self, p: &Self) -> f64;
}

// 二次元の点
struct Point {
    x: f64, y: f64
}

impl Point {
    fn new(x1: f64, y1: f64) -> Point {
        Point {x: x1, y: y1}
    }
}

// Distance の実装
impl Distance for Point {
    fn distance(&self, p: &Point) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        (dx * dx + dy * dy).sqrt()
    }
}

// 三次元の点
struct Point3D {
    x: f64, y: f64, z: f64
}

impl Point3D {
    fn new(x1: f64, y1: f64, z1: f64) -> Point3D {
        Point3D { x: x1, y: y1, z: z1 }
    }
}

// Distance の実装
impl Distance for Point3D {
    fn distance(&self, p: &Point3D) -> f64 {
        let dx = self.x - p.x;
        let dy = self.y - p.y;
        let dz = self.z - p.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

// ２点間の距離を表示する
// T は Distance を実装しているデータ型に限定される
fn print_distance<T: Distance>(p1: &T, p2: &T) {
    println!("{:.8}", p1.distance(p2));
}

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(10.0, 10.0);
    let p3 = Point3D::new(0.0, 0.0, 0.0);
    let p4 = Point3D::new(10.0, 10.0, 10.0);
    print_distance(&p1, &p2);
    print_distance(&p3, &p4);
}

main();
}

println!("--------------------  generic 型パラメータの境界");
{
// 配列の探索
fn find<T: PartialEq + Copy>(key: T, data: &[T]) -> bool {
    for &x in data {
        if key == x { return true; }
    }
    false
}

// 連想リストの探索
fn assoc<T: PartialEq + Copy, U>(key: T, data: &[(T, U)]) -> Option<&U> {
    for &(x, ref v) in data {
        if key == x { return Some(v); }
    }
    None
}

fn main() {
    let data = [1,2,3,4,5,6,7,8];
    println!("{}", find(8, &data));
    println!("{}", find(0, &data));

    let data1 = ["foo", "bar", "baz"];
    println!("{}", find("baz", &data1));
    println!("{}", find("oops", &data1));

    let data2 = [("foo", 100), ("bar", 200), ("baz", 300)];
    match assoc("baz", &data2) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    match assoc("oops", &data2) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
}
main();
}


}

