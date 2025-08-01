
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  Iterator ");
{
// start 以上 end 未満の整数列を生成する
struct IntSeq {
    start: i32, end: i32
}

// 初期化
impl IntSeq {
    fn new(s: i32, e: i32) -> IntSeq {
        IntSeq { start: s, end: e }
    }
}

// イテレータの実装
impl Iterator for IntSeq {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start < self.end {
            let x = self.start;
            self.start += 1;
            Some(x)
        } else {
            None
        }
    }
}

fn main() {
    let mut s0 = IntSeq::new(0, 5);
    while let Some(x) = s0.next() {
        println!("{}", x);
    }
    for x in IntSeq::new(5, 10) {
        println!("{}", x);
    }
    let mut s1 = 10 .. 15;
    while let Some(x) = s1.next() {
        println!("{}", x);
    }
}
main();
}
println!("--------------------  Iterator  map, filter, fold, collect の簡単な使用例");
{
fn change(x: i32) -> String {
    if x % 15 == 0 {
        "FizzBuzz".to_string()
    } else if x % 3 == 0 {
        "Fizz".to_string()
    } else if x % 5 == 0 {
        "Buzz".to_string()
    } else {
        format!("{}", x)
    }
}

fn main() {
    let xs: Vec<_> = (1..101).map(change).collect();
    println!("{:?}", xs);
    let ys = vec![1,2,3,4,5,6,7,8,9,10];
    {
        let a: Vec<_> = ys.iter().map(|x| x * x).collect();
        println!("{:?}", a);
        let b: Vec<_> = ys.iter().filter(|&x| x % 2 == 0).collect();
        println!("{:?}", b);
        println!("{}", ys.iter().fold(0, |a, x| a + x));
    }
    let c: Vec<_> = ys.into_iter().filter(|x| x % 2 == 1).collect();
    println!("{:?}", c);
    // println!("()", ys);   コンパイルエラー
}
main();
}

}

