/*
 * https://www.tohoho-web.com/ex/rust.html
 * とほほのRust入門
*/
use regex::Regex;
mod _03_hw;
mod _09_type;

struct  Entry{
    x: i32,
    y: i32,
    z: i32,
}

struct  Func<'a>{
    name: &'a str,
    func: fn(),
}

fn test1() {
   println!("call test1");
}
fn test2() {
   println!("call test2");
}
fn test3() {
   println!("call test3");
}

fn main() {

    let entry_array: &[Entry] = &[ Entry{ x:100, y:200,z:300},
                                   Entry{ x:101, y:201,z:301},
                                   Entry{ x:102, y:202,z:302}
                                 ];
    for entry in entry_array {
        println!("{}", entry.x);
    }

    let func_array: &[Func] = &[ Func{ name: "T1", func: test1},
                                 Func{ name: "T2", func: test2},
                                 Func{ name: "T3", func: test3}
                                 ];
    for entry in func_array {
        println!("{}", entry.name);
        (entry.func)();
    }

/*
    let test_id = "09";
    //let test_id = "*";

    match test_id {

     "03" => _03_hw::main() ,
     "09" => _09_type::main() ,

      _       => println!("_")
    }
*/
/*
let restr = "_";
//let re = Regex::new(r"03").unwrap();
let re = Regex::new(restr).unwrap();
if re.is_match("_03_hw") {
     _03_hw::main() ;
}
*/

}
