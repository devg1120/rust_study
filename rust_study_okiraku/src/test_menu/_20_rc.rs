
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {
/*
Rc<T> は Box<T> に「参照カウンタ (Reference Count)」を付けたもの
複数の変数から Rc が保持しているヒープ上のデータを共有することができる
Rc を clone() すると、Rc が保持しているデータはコピーされずに、参照カウンタが +1 される
Rc を保持している変数がスコープから外れると、参照カウンタが -1 される
参照カウンタが 0 になったとき、Rc が保持しているヒープ上のデータが解放される
Rc<T> の動作は C++ の shared_ptr<T> に近い
Rc<T> にも Deref<Target=T> が実装されている
*/


println!("--------------------  Rc<T>");
{
use std::rc::Rc;

#[derive(Clone)]
struct Foo {
    num: i32
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Drop foo {}", self.num);
    }
}

fn main() {
    let a = Rc::new(Foo { num: 123 });
    println!("{}", Rc::strong_count(&a));
    let x = Box::new(Foo { num: 456 });
    println!("{}", x.num);
    {
        let b = a.clone();
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(&b));
        let c = &a;
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(c));
        let y = x.clone();
        println!("{}", y.num);
    }
    println!("{}", Rc::strong_count(&a));
}
main();
}
println!("--------------------  ");
{
use std::rc::Rc;

#[derive(Debug)]
struct Foo {
    num: i32
}

fn main() {
    let a = Box::new(Foo { num: 123 });
    let b = *a;   // move
    // println!("{}", a.num); コンパイルエラー
    println!("{}", b.num);

    let c = Rc::new(Foo { num: 456 });
    // let x = *c;    Rc の場合は単純に move することはできない
    {
        let d = c.clone();
        // 参照カウンタが 1 よりも多いと Err
        match Rc::try_unwrap(d) {
            Ok(x) => println!("{}", x.num),
            Err(x) => println!("error {:?}", x)
        }
    }
    // 参照カウンタが 1 ならば Ok
    match Rc::try_unwrap(c) {
        Ok(x) => println!("{}", x.num),
        Err(x) => println!("error {:?}", x)
    }
}
main();
}
println!("--------------------  連結リスト");
{
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Nil,
    Cons(i32, Rc<List>)
}

use List::*;

impl List {
    fn new() -> Rc<List> {
        Rc::new(Nil)
    }
    fn cons(x: i32, xs: &Rc<List>) -> Rc<List> {
        Rc::new(Cons(x, xs.clone()))
    }
}

impl Drop for List {
    fn drop(&mut self) {
        match *self {
            Nil => println!("drop Nil"),
            Cons(x, _) => println!("drop {}", x)
        }
    }
}

fn main() {
    let a = List::new();
    println!("{:?}", a);
    {
        let b = List::cons(1, &a);
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(&b));
        {
            let c = List::cons(2, &b);
            println!("{:?}", a);
            println!("{:?}", b);
            println!("{:?}", c);
            println!("{}", Rc::strong_count(&a));
            println!("{}", Rc::strong_count(&b));
            println!("{}", Rc::strong_count(&c));
        }
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(&b));
    }
    println!("{}", Rc::strong_count(&a));
}
main();
}

}

