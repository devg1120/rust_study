
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
println!("--------------------  Drop");
{
struct Droppable {
    name: &'static str,
}

// このちょっとした実装で、`drop`にコンソール出力機能がつきます。
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // `drop`関数を用いて変数を手動で開放することもできます。
    drop(_a);
    // TODO ^ この行をコメントアウトしてみましょう。

    println!("end of the main function");

    // `_a`はここで`drop`されることは *ありません* 。なぜならば、上ですでに
    // （手動で）`drop`されているためです。
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
}

