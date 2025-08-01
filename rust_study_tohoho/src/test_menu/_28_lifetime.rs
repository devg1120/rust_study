
/*
 * ライフタイム というのは参照が有効になるスコープ
 */

pub fn main() {

// ライフタイムは&演算子の後ろに指定します．慣例的に a,b,c,… と指定します．

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]


println!("--------------------  lifetime 1");
{
    let r; 

    {
        let x = 5;
        r = &x;
    }  // xがdropされる

  // println!("r is {}", r); //Error  dropされたxを参照してるr君
}

println!("--------------------  lifetime 2");
{

    // 戻り値の参照がどちらの引数の参照をしてるか、コンパイル時に判断不能
    // Lifetime Annotationつけてあげる
   
 // fn longest(x: &str, y: &str) -> &str {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

println!("--------------------  lifetime 3");
{
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

  // string1 string2 短い方のlifetimeを見る
  let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
	// resultはstring1なので
       
        result = longest(string1.as_str(), string2.as_str());
    }
  // println!("The longest string is {}", result);   // Error   string2はDrop
}

println!("--------------------  lifetime 4");
{
   #[derive(Debug)]
   //struct ImportantExcerpt {   // Error
   //    part: &str,
   //}
   struct ImportantExcerpt<'a> { // Ok
       part: &'a str,
   }
   fn main() {
       let novel = String::from("Call me Ishmael. Some years ago...");
       let first_sentence = novel.split('.')
           .next()
           .expect("Could not find a '.'");  // '.'が見つかりませんでした
       let i = ImportantExcerpt { part: first_sentence };
       println!("ImportantExcerpt is {:?}", i)   
   }
   main()
}
}

