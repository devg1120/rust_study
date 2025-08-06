
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

trait Speak {
  fn speak(&self);
}


println!("--------------------  トレイトオブジェクト Dog");
{
struct Dog;

impl Speak for Dog {
  fn speak(&self) {
    println!("Woof!");
  }
}

fn main() {
  // このanimalがトレイトオブジェクト
  let animal: Box<dyn Speak> = Box::new(Dog);
  animal.speak();
}
main();
}

println!("--------------------  トレイトオブジェクト Dog Cat");
/*
トレイトオブジェクトを使うとポリモーフィズムが実現できる
*/
{
struct Dog;
struct Cat;

impl Speak for Dog {
  fn speak(&self) {
    println!("Woof!");
  }
}

impl Speak for Cat {
  fn speak(&self) {
    println!("Meow!");
  }
}

fn main() {
  let animals: Vec<Box<dyn Speak>> = vec![Box::new(Dog), Box::new(Cat)];
  // 実際に中に入っている型は異なるが、同じSpeakトレイトを実装しているため、speakメソッドを呼べる
  for animal in animals {
    animal.speak();
  }
}
main();
}
println!("--------------------  トレイト境界");
/*
ジェネリクス型のパラメタを使い特定のトレイトが実装されているものだけに制限(制約)を掛ける
*/
{
struct Dog;

impl Speak for Dog {
  fn speak(&self) {
    println!("Woof!");
  }
}

// Speakトレイトを実装している構造体のみこの関数にわたすことができる
fn speak_animal<T: Speak>(animal: T) {
  animal.speak();
}

fn main() {
    speak_animal(Dog);
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

