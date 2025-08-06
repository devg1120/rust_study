
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::Display;
use std::fmt::Debug;

/*
#[derive(Debug)]
println!("{:#?}", foo);
*/
pub fn main() {



pub trait Summary {

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("@{}...", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct Post {
    pub title: String, // タイトル
    pub author: String, // 著者
    pub content: String, // 内容
}

impl Summary for Post {
   fn summarize_author(&self) -> String {
     format!("{}", self.author)
   }
   fn summarize(&self) -> String {
     format!("{}：{}", self.author, self.content)
   }
}
impl Summary for Tweet {
   fn summarize_author(&self) -> String {
     format!("{}", self.username)
   }
   fn summarize(&self) -> String {
     format!("@{}：{}", self.username, self.content)
   }
}


println!("--------------------  Trait の定義");
{

fn main() {
   let tweet = Tweet {
       username: String::from("haha"),
       content: String::from("the content"),
       reply: false,
       retweet: false,
   };
   println!("{}",  tweet.summarize())
}

main();
}

println!("--------------------  rait を関数の引数として使用する");
/*
Summary を実装している任意の型をこの関数の引数として渡すことができ
関数内ではその Trait に定義されたメソッドを呼び出すことができます。
*/
{
pub fn notify(item: &impl Summary) { // Trait を引数として指定
    println!("Breaking news! {}", item.summarize());
}
pub fn notif2<T: Summary> (item: &T) { // 完全な記述形式
    println!("Breaking news! {}", item.summarize());
}

let tweet = Tweet {
       username: String::from("haha"),
       content: String::from("the content"),
       reply: false,
       retweet: false,
};
notify(&tweet);

let post =  Post {
    title: String::from("タイトル"),
    author: String::from("著者"),
    content: String::from("内容"),
};
notify(&post);

}
println!("--------------------  Trait Bound（トレイト境界） + where");
{

pub fn notify1(item: &impl Summary) { // Trait を引数として指定
    println!("Breaking news! {}", item.summarize());
}
pub fn notif2<T: Summary> (item: &T) { // 完全な記述形式
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {} // impl Trait を使用
pub fn notify4<T: Summary>(item1: &T, item2: &T) {}  // ジェネリック T を使った場合：item1 と item2 は同じ型で、T は Summary を実装している必要がある


fn some_function<T, U>(t: &T, u: &U) -> i32
where T: Display + Clone  ,
      U: Clone + Debug   {
          100
}
//let ans = some_function(&100,&99);
let ans = some_function(&String::from("abcd"), &String::from("XYZ"));
println!("{}",ans);
}

println!("--------------------  Trait 境界で条件付きにメソッドや Trait を実装する");
{
fn notify(summary: impl Summary) {
    println!("notify: {}",  summary.summarize())
}

fn notify_all(summaries: Vec<impl Summary>) {
    for summary in summaries {
        println!("notify: {}",  summary.summarize())
    }
}

fn main() {
   let tweet1 = Tweet {
       username: String::from("haha"),
       content: String::from("the content"),
       reply: false,
       retweet: false,
   };
   let tweet2 = Tweet {
       username: String::from("ggg"),
       content: String::from("the ------"),
       reply: false,
       retweet: false,
   };
   let post =  Post {
       title: String::from("タイトル"),
       author: String::from("著者"),
       content: String::from("内容"),
   };
   let all = vec![tweet1, tweet2];
   notify_all(all);

}
main();
}
println!("--------------------  Trait オブジェクト");
/*
特定の Trait を実装しているかどうかだけを気にして、実際の具体的な型に依存しないようにしたい場合は、
スマートポインタ Box とキーワード dyn を組み合わせた Trait オブジェクトを使用することができます
*/
{
fn notify(summary: Box<dyn Summary>) {
    println!("notify: {}",  summary.summarize())
}

fn notify_all(summaries: Vec<Box<dyn Summary>>) {
    for summary in summaries {
        println!("notify: {}",  summary.summarize())
    }
}

fn main() {
   let tweet1 = Tweet {
       username: String::from("haha"),
       content: String::from("the content"),
       reply: false,
       retweet: false,
   };
   let tweet2 = Tweet {
       username: String::from("ggg"),
       content: String::from("the ------"),
       reply: false,
       retweet: false,
   };
   let post =  Post {
       title: String::from("タイトル"),
       author: String::from("著者"),
       content: String::from("内容"),
   };
   let all: Vec<Box<dyn Summary>> = vec![
                        Box::new(tweet1), 
                        Box::new(tweet2),
                        Box::new(post),
                     ];
   notify_all(all);
}
main();
}
println!("--------------------  ジェネリックでの Trait の使用");
/*
impl Summary はジェネリックプログラミングにおける「トレイト境界」のシンタックスシュガーであり
以下のように書き換えることができます
*/
{
fn notify<T: Summary>(summary: T) {
    println!("notify: {}",  summary.summarize())
}

fn notify_all<T: Summary>(summaries: Vec<T>) {
    for summary in summaries {
        println!("notify: {}",  summary.summarize())
    }
}

fn main() {
   let tweet = Tweet {
       username: String::from("haha"),
       content: String::from("the content"),
       reply: false,
       retweet: false,
   };
   let tweets = vec![tweet];
   notify_all(tweets);
}
main();
}
println!("--------------------  関数の戻り値としての impl Trait");
{
fn notify_all(summaries: Vec<impl Summary>) {
    for summary in summaries {
        println!("notify: {}",  summary.summarize())
    }
}
fn returns_summarizable() -> impl Summary {
    Tweet {
       username: String::from("haha"),
       content: String::from("the content"),
       reply: false,
       retweet: false,
   }
}
fn main() {

  let a = returns_summarizable();
  let b = returns_summarizable();
   let all = vec![a, b];
   notify_all(all);

}
main();
}
println!("--------------------  数の戻り値としての impl Trait 異なる型");
{
fn notify_all(summaries: Vec<Box<dyn Summary>>) {
    for summary in summaries {
        println!("notify: {}",  summary.summarize())
    }
}
fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(
                   Tweet {
                       username: String::from("ggg"),
                       content: String::from("the ------"),
                       reply: false,
                       retweet: false,
                   }
            ) // Trait オブジェクト
    } else {
        Box::new(
                   Post {
                       title: String::from("タイトル"),
                       author: String::from("著者"),
                       content: String::from("内容"),
                   }
            )  // Trait オブジェクト
    }
}

fn main() {

  let a = returns_summarizable(true);
  let b = returns_summarizable(true);
  let c = returns_summarizable(false);
  let all = vec![a, b, c];
  notify_all(all);

}
main();
}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
}

