
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  VecDeque<T>");
/*
std::collections::VecDeque は「両端キュー (double ended queue: deque)」のこと
VecDeque は VecDeque::new() で生成する
push_back(item) で末尾にデータを追加する
push_front(item) で先頭にデータを追加する
pop_back() -> Option<T> で末尾からデータを取り出す
pop_front() -> Option<T> で先頭からデータを取り出す
front() -> Option<&T> は先頭要素への参照を返す
back() -> Option<&T> は末尾要素への参照を返す
get(n) -> Option<&T> は n 番目の要素への参照を返す
iter() は先頭からデータを順番に取り出すイテレータを生成する
len() は VecDeque に格納されているデータ数を返す
is_empty() は VecDeque が空であれば true を返す
clear() は VecDeque を空にする
*/
{
use std::collections::VecDeque;

fn main() {
    // スタック (Stack) の動作
    let mut s = VecDeque::new();
    for x in 0 .. 10 {
        s.push_back(x);
    }
    println!("{:?}", s);
    while !s.is_empty() {
        println!("{}", s.pop_back().unwrap());
    }
    // キュー (Queue) の動作
    let mut q = VecDeque::new();
    for x in 10 .. 20 {
        q.push_back(x);
    }
    println!("{:?}", q);
    while !q.is_empty() {
        println!("{}", q.pop_front().unwrap());
    }
}
main();
}
println!("--------------------  LinkedList<T>");
/*
std::collections::LinkedList<T> は「双方向連結リスト (doubly linked list)」のこと
LinkedList は LinkedList::new() で生成する
VecDeque のように「両端キュー」として使用できる
両端キューとして使用する場合、メソッドは VecDeque と同じ
LinkedList にメソッド get(n) は用意されていない
*/
{
use std::collections::LinkedList;

fn main() {
    // スタック (Stack) の動作
    let mut s = LinkedList::new();
    for x in 0 .. 10 {
        s.push_back(x);
    }
    println!("{:?}", s);
    while !s.is_empty() {
        println!("{}", s.pop_back().unwrap());
    }
    // キュー (Queue) の動作
    let mut q = LinkedList::new();
    for x in 10 .. 20 {
        q.push_back(x);
    }
    println!("{:?}", q);
    while !q.is_empty() {
        println!("{}", q.pop_front().unwrap());
    }
}
main();
}
println!("--------------------  HashMap<K, V> ");
/*
std::collections::HashMap<K, V> は「連想配列」
*/
{
use std::collections::HashMap;

fn main() {
    let mut ht = HashMap::new();
    ht.insert("foo", 1);
    ht.insert("bar", 2);
    ht.insert("baz", 3);
    ht.insert("oops", 4);
    println!("{:?}", ht);
    println!("{}", ht.contains_key("baz"));
    println!("{}", ht.contains_key("Baz"));
    println!("{:?}", ht.get("foo"));
    println!("{:?}", ht.get("Foo"));
    println!("{:?}", ht.remove("Oops"));
    println!("{:?}", ht.remove("oops"));
    println!("{:?}", ht.insert("foo", 100));
    println!("{:?}", ht.insert("oops", 200));
    for (k, v) in ht.iter() {
        println!("{}, {}", k, v);
    }
}
main();
}
println!("--------------------  HashSet<T>");
/*
std::collections::HashSet<T> は集合を表すコレクション
*/
{
use std::collections::HashSet;

fn main() {
    let a: HashSet<_> = [1,2,3,4].iter().cloned().collect();
    let b: HashSet<_> = [3,4,5,6].iter().cloned().collect();
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a.contains(&1));
    println!("{}", b.contains(&1));
    let c: HashSet<_> = a.union(&b).cloned().collect();
    println!("{:?}", c);
    let d: HashSet<_> = a.intersection(&b).collect();
    println!("{:?}", d);
    let e: HashSet<_> = a.difference(&b).collect();
    println!("{:?}", e);
    println!("{}", a.is_subset(&c));
    println!("{}", b.is_subset(&c));
    println!("{}", a.is_subset(&b));
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

