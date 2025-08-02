
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {
/*
Cell<T> は immutable なデータの中で mutable な操作をするときに使用する
T には Copy 型の制約がある
Cell はメソッド Cell::new(値) で生成する
メソッド get() でデータを読み取り、メソッド set() でデータを書き換える
*/

println!("--------------------  Cell<T>");
{
use std::cell::Cell;

struct Foo {
    x: i32,
    y: Cell<i32>
}

fn main() {
    let a = Cell::new(123);
    println!("{:?}", a);
    println!("{}", a.get());
    a.set(456);
    println!("{:?}", a);
    println!("{}", a.get());

    let b = Foo { x: 789, y: Cell::new(999) };
    println!("{}. {:?}", b.x, b.y);
    b.y.set(1000);
    println!("{}, {:?}", b.x, b.y);

    let c = Box::new(Cell::new(1.234));
    println!("{}", c.get());
    c.set(5.678);
    println!("{}", c.get());
}
main();
}
println!("--------------------  RefCell<T>");
{
use std::cell::RefCell;

#[derive(Debug)]
struct Foo {
    x: i32,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop foo {}", self.x);
    }
}

#[derive(Debug)]
struct Bar {
    foo: RefCell<Foo>
}

fn main() {
    let a = RefCell::new(123);
    println!("{:?}", a);
    {
        println!("{}", *a.borrow());  // immutable な参照
    }
    {
        let mut ref_a = a.borrow_mut();   // mutable な参照
        *ref_a = 456;
        // println!("{}", *a.borrow());  immutable な参照を借りるとパニック
    }
    println!("{}", *a.borrow());  //immutable な参照
    
    let b = Bar { foo: RefCell::new(Foo { x: 123 })};
    println!("{:?}", b);
    {
        println!("{:?}", *b.foo.borrow());  // immutable な参照
    }
    {
        let mut ref_b = b.foo.borrow_mut();   // mutable な参照
        *ref_b = Foo { x: 456 };              // Foo { x: 123 } は廃棄される
    }
    println!("{:?}", b);

    let c = Box::new(RefCell::new(Foo { x: 789 }));
    println!("{:?}", c);
    {
        println!("{}", c.borrow().x);     // フィールド x の値を参照する
        c.borrow_mut().x = 999;           // フィールド x の値を書き換える
        println!("{:?}", c);
    }
    {
        let mut ref_c = c.borrow_mut();   // mutable な参照
        *ref_c = Foo { x: 1000 };         // Foo { x: 999 } は廃棄される
    }

    let d = c.into_inner();    // メソッド into_inner() は中身のデータを move する
    // println!("{:?}", c);    コンパイルエラーになる
    println!("{:?}", d);
}
main();
}
println!("--------------------  ");
{
use std::cell::Cell;

struct Foo {
    num: Cell<i32>
}

impl Foo {
    fn new(x: i32) -> Foo {
        Foo { num: Cell::new(x) }
    }
    fn get_num(&self) -> i32 {
        self.num.get()
    }
    fn set_num(&self, x: i32) {   // immutable な参照でも更新できる
        self.num.set(x)
    }
}

fn main() {
    let a = Foo::new(100);
    let b = &a;
    let c = &a;
    println!("{}", a.get_num());
    println!("{}", b.get_num());
    println!("{}", c.get_num());
    c.set_num(200);
    println!("{}", a.get_num());
    println!("{}", b.get_num());
    println!("{}", c.get_num());
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

