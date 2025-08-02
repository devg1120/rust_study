
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  トレイトの基本");
/*
 Rust の「トレイト (trait)」はメソッドの仕様を定義したもの
トレイトは「データ型の振る舞い (機能)」に名前を付けたものと考えることができる
データ型 A, B がトレイト X を実装していたらならば、A, B を同じデータ型として扱うことができる
これを「トレイトオブジェクト」という (あとで説明する)
トレイトは「ジェネリクス」で境界条件を設定するときにも使用する (あとで説明する)
トレイトはキーワード trait で定義する
*/
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

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(10.0, 10.0);
    let p3 = Point3D::new(0.0, 0.0, 0.0);
    let p4 = Point3D::new(10.0, 10.0, 10.0);
    println!("{}", p1.distance(&p2));
    println!("{}", p3.distance(&p4));    
}
main();
}

println!("--------------------  トレイトの継承");
/*
トレイトは他のトレイトを継承することができる
  trait A : B { ... }
トレイト A を実装するとき、トレイト B も実装しないとコンパイルエラーになる
*/
{
trait Foo {
    fn method_a(&self);
}

trait Bar : Foo {
    fn method_b(&self);
}

struct Baz;

impl Foo for Baz {
    fn method_a(&self) {
        println!("method_a!");
    }
}

impl Bar for Baz {
    fn method_b(&self) {
        println!("method_b!");
    }
}

fn main() {
    let a = Baz;
    a.method_a();
    a.method_b();
}
main();
}

println!("--------------------  Derive");
/*
Rust は #[derive()] というアトリビュートを使って、特定のトレイトの標準的な機能を実装することができる
カッコの中にトレイトを指定する
Derive できるトレイト
   Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd
*/

{
// 距離を求める
trait Distance {
    fn distance(&self, p: &Self) -> f64;
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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

fn main() {
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(10.0, 10.0);
    let p3 = Point3D::new(0.0, 0.0, 0.0);
    let p4 = Point3D::new(10.0, 10.0, 10.0);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{:?}", p4);
    println!("{}", p3 == p4);
    println!("{}", p3 != p4);
    println!("{}", p4 == p4);
    println!("{}", p4 != p4);
    println!("{}", p1.distance(&p2));
    println!("{}", p3.distance(&p4));
}
main();

}

println!("--------------------  トレイト境界");
/*
ジェネリクスの型パラメータには「境界」を設定することができる
Rust では「トレイト境界」とか「ジェネリック境界」と呼ばれているようだ
<T: U> とした場合、T はトレイト U を実装しているデータ型に制限される
複数のトレイトを境界に指定したい場合は <T: U + V> のように + でつなげる
境界は where 句 (where T: U, ...) を使って設定することもできる
where 句は本体 {...} の直前に挿入する
*/

{
// 距離を求める
trait Distance {
    fn distance(&self, p: &Self) -> f64;
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{:?}", p4);
    print_distance(&p1, &p2);
    print_distance(&p3, &p4);
}
main();

}
}

