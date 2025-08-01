
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


println!("--------------------  lifetime None");

//| fn func1(x: &str, y: &str) -> &str {
//|             ----     ----     ^ expected named lifetime parameter
//| fn func2(x: &str, y: &str) -> &str {
//|             ----     ----     ^ expected named lifetime parameter
                                   // 設計者として生存要求lifetimeを明記するよう指摘
{
/*
fn func1(x: &str, y: &str) -> &str {
    println!("{x} {y}");
    x
}

fn func2(x: &str, y: &str) -> &str {
    println!("{x} {y}");
    y
}
fn main() {
    let s1: &str;
    let s2: &str;
    let hello = "Hello".to_string();
    {
        let world = "World!".to_string();
        s1 = func1(&hello, &world);      // <-- (1)
        s2 = func2(&hello, &world);      // <-- (2)
    }                                    // <-- (3)
    println!("{}", s1);                  // <-- (4)
    println!("{}", s2);                  // <-- (5)
}
main();
*/
}

println!("--------------------  lifetime Set");    
//|         s2 = func2(&hello, &world);      // <-- (2)
//|                            ^^^^^^ borrowed value does not live long enough
                                     // 修正が必要な個所を適切に指摘される
{
/*
    // xの参照先は返値よりも先に死んではならない
fn func1<'a,'b>(x: &'a str, y: &'b str) -> &'a str {
    println!("{x} {y}");
    x
}
    // yの参照先は返値よりも先に死んではならない
fn func2<'a,'b>(x: &'a str, y: &'b str) -> &'b str {
    println!("{x} {y}");
    y
}
fn main() {
    let s1: &str;
    let s2: &str;
    let hello = "Hello".to_string();
    {
        let world = "World!".to_string();
        s1 = func1(&hello, &world);      // <-- (1)
        s2 = func2(&hello, &world);      // <-- (2)
    }                                    // <-- (3)
    println!("{}", s1);                  // <-- (4)
    println!("{}", s2);                  // <-- (5)
}
main();
*/
}
println!("--------------------  lifetime Ok");
{
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
main();
}
println!("-------------------- lifetime Error ");
//|         result = longest(string1.as_str(), string2.as_str());
//|                                            ^^^^^^^ borrowed value does not live long enough

{
/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
main();
*/

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

