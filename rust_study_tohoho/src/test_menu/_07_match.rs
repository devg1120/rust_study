/*
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Debug)]
*/


pub fn main() {

    println!("-------------------- match 1");
    {
        #[allow(dead_code)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        println!("{}",value_in_cents(Coin::Dime));
    }

    println!("-------------------- match 2");
    {
         fn plus_one(x: Option<i32>) -> Option<i32> {
             match x {
                 None => None,
                 Some(i) => Some(i + 1),
             }
         }
         
         let five = Some(5);
         let six = plus_one(five);
         let num =    match six {
                 None => 0 ,
                 Some(i) => i 
             };
         println!("{}", num);
    }

    println!("-------------------- match 3 wildcard");
    {
        let some_u8_value = 3u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }
    }
    println!("-------------------- match 4 ref");
    {
       let a: Option<String> = Some(String::from("hello"));
       match a {
          Some(ref x) => println!("{}", x), // reference
          None => ()
       }
       println!("{:?}", a); // borrow check!! - OK
    }
    println!("-------------------- match 5 return obj");
    {
       let a: Option<String> = Some(String::from("hello"));
       let a = match a {
          Some(x) => { println!("{}", x); Some(x) }
          None => None,
       };
       println!("{:?}", a); // borrow check!! - OK
    }
    println!("-------------------- match 6 match gard");
    {
       let num = Some(4);
       match num {
           Some(x) if x < 5 => println!("less than five: {}", x),
           Some(x) => println!("{}", x),
           None => (),
       }
    }
    println!("-------------------- match 4");
    {
    }
}

