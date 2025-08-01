
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {

/*
トレイトオブジェクトはプログラムの実行時にメソッドを選択する仕組み (動的ディスパッチ)
C++ の仮想関数によく似ている
トレイトを Foo とすると、トレイトオブジェクトのデータ型は &dyn Foo, &mut dyn Foo, Box<dyn Foo> (Rust 2021)
トレイトオブジェクトはトレイトを実装したデータ型を「型キャスト」するか「型強制」することで生成する
*/

println!("--------------------  trail object ");
{
trait Foo {
    fn method(&self);
}

struct Bar;
struct Baz;

impl Foo for Bar {
    fn method(&self) { println!("Bar method"); }
}

impl Foo for Baz {
    fn method(&self) { println!("Baz method"); }
}

fn call_method(func: &dyn Foo) {
    func.method();    // 動的ディスパッチ
}

fn call_method_box(func: Box<dyn Foo>) {
    func.method();
}

fn main() {
    let x = Bar;
    let y = Baz;
    call_method(&x as &dyn Foo);
    call_method(&y);
    call_method_box(Box::new(x));
    call_method_box(Box::new(y));    
}
main();
}

println!("--------------------  trail object");
{
// 図形のトレイト
trait Figure {
    fn area(&self) -> f64;
    fn kind(&self) -> &str;

    // デフォルトメソッド
    fn print(&self) {
        println!("{}: area = {:.3}", self.kind(), self.area());
    }
}

// 三角形
struct Triangle {
    altitude: f64, base_line: f64
}

impl Triangle {
    fn new(a: f64, b: f64) -> Triangle {
        Triangle { altitude: a, base_line: b }
    }
}

impl Figure for Triangle {
    fn area(&self) -> f64 {
        self.altitude * self.base_line / 2.0
    }
    fn kind(&self) -> &str {
        "Triangle"
    }
}

// 四角形
struct Rectangle {
    width: f64, height: f64
}

impl Rectangle {
    fn new(w: f64, h: f64) -> Rectangle {
        Rectangle { width: w, height: h}
    }
}

impl Figure for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    
    fn kind(&self) -> &str {
        "Rectangle"
    }
}

// 円
struct Circle {
  radius: f64
}

impl Circle {
    fn new(r: f64) -> Circle {
        Circle { radius: r }
    }
}

impl Figure for Circle {
    fn area(&self) -> f64 {
         std::f64::consts::PI * self.radius * self.radius
    }

    fn kind(&self) -> &str {
        "Circle"
    }
}

// 図形の合計値を求める
fn figure_sum(data: &[&dyn Figure]) -> f64 {
    let mut a = 0.0;
    for fig in data {
        a += fig.area();
    }
    a
}

fn main() {
    let a = Triangle::new(2.0, 2.0);
    let b = Rectangle::new(2.0, 2.0);
    let c = Circle::new(2.0);
    a.print();
    b.print();
    c.print();

    // トレイトオブジェクトを配列に格納する
    let d = [&a as &dyn Figure, &b as &dyn Figure, &c as &dyn Figure];
    println!("{:.3}", figure_sum(&d));
}
main();
}
}

