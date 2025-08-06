
mod test_menu;
use crate::test_menu::TEST_ARRAY;
use crate::test_menu::REF;

use regex::Regex;
use std::env;


fn menu_print() {
    print!("{}","---------------------------------------------");
    print!("{}", REF);
    print!("{}","---------------------------------------------\n");
    for entry in TEST_ARRAY {
        println!("{}", entry.name);
    }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let mut restr = "";
    if args.len() == 2 {
         //println!("{:?}", args[1]);
         restr = &args[1];
    }
    if restr == "menu" || restr == "M" { menu_print(); std::process::exit(0);}
    let re = Regex::new(restr).unwrap();

    println!("");
    println!("*** Rust TESTTING CASE /{}/ ***", restr);
    println!("");
    let mut m: bool = false;
    for entry in TEST_ARRAY {
        //println!("{}", entry.name);
        //(entry.func)();
        if re.is_match(entry.name) {
             m = true;
             println!("********************************* {}\n", entry.name);
             (entry.func)();
        }
    }
    if  !m  {
        println!("...Error not match entry")
    }

    //rust_study::run();
}
