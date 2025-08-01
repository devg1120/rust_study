
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  ライフタイムパラメータの指定");
{
// 配列の探索
fn find<'a, 'b, T: PartialEq>(item: &'b T, xs: &'a [T]) -> Option<&'a T> {
    for x in xs {
        if x == item { return Some(x); }
    }
    None
}

fn main() {
    let xs = [1,2,3,4,5,6,7,8];
    for x in 0 .. 10 {
        match find(&x, &xs) {
            Some(v) => println!("{}", v),
            None => println!("None")
        }
    }
}
main();
}
println!("--------------------   構造体のライフタイム");
{
struct Foo<'a> {
    x: &'a i32
}

fn main() {
    let y = 123;
    let z = Foo { x: &y };
    println!("{}", z.x);
    // let z1;  コンパイルエラー
    // {
    //     let y1 = 456;
    //     z1 = Foo { x: &y1 };
    // }
    // println!("{}", z1.x);
}
main();
}
println!("--------------------  メソッドのライフタイム");
{
struct Foo<'a> {
    x: &'a i32
}

impl <'a> Foo<'a> {
    fn foo(&self) ->&i32 { self.x }

    fn foo1(&self, y: &i32) ->&i32 {
        println!("{},{}", self.x, y);
        self.x
    }    

    fn foo2<'b>(&self, y: &'b i32) ->&'b i32 {
        println!("{},{}", self.x, y);
        y
    }    
}

fn main() {
    let y = 123;
    let z = Foo { x: &y };
    println!("{}", z.foo());
    let y1 = 456;
    println!("{}", z.foo1(&y1));
    println!("{}", z.foo2(&y1));
}
main();
}

}

