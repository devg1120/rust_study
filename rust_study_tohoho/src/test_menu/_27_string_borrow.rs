
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
fn print_type<T>(_: T) {
    println!("type: {}",   std::any::type_name::<T>());
}


pub fn main() {
   println!("-------------------- &str");
   {
       let s1 = "hello";
       let s2 = s1;
       println!("s1 -> {}",s1);
       print_type(s1);
       println!("s2 -> {}",s2);
       print_type(s2);
   }

   println!("-------------------- String");
   {
       let s1 = String::from("hello");
       println!("s1 -> {}",s1);
       let s2 = s1;
       //println!("s1 -> {}",s1);   // Error
       println!("s1 -> {}","...error");  
   }

   println!("-------------------- String ref");
   {
       let s1 = String::from("hello");
       let s2 = &s1;                   // ref
       println!("s1 -> {}",s1);   
       println!("s2 -> {}",s2);   
   }

   println!("-------------------- String mut borrow");
   {
       let mut s1 = String::from("hello");
       s1.push_str("_xyz");
       let s2 = &mut s1;
       s2.push_str("_abc");

      // println!("s1 -> {}",s1);   //          error
      // println!("s2 -> {}",s2);   // mut

       println!("s2 -> {}",s2);     // mut
       println!("s1 -> {}",s1);     //
   }
   println!("-------------------- String take return");
   {
       fn main() {
           let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                               // ムーブする
       
           let s2 = String::from("hello");     // s2がスコープに入る
       
           let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                               // 戻り値もs3にムーブされる
            println!("s1: {}",s1);
            //println!("s2: {}",s2); //Error
            println!("s3: {}",s3);
       } // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
         // 何も起きない。s1もスコープを抜け、ドロップされる。
       
       fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                                    // 呼び出した関数にムーブする
       
           let some_string = String::from("hello"); // some_stringがスコープに入る
       
           some_string                              // some_stringが返され、呼び出し元関数に
                                                    // ムーブされる
       }
       
       // takes_and_gives_backは、Stringを一つ受け取り、返す。
       fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
       
           a_string  // a_stringが返され、呼び出し元関数にムーブされる
       }
       main();
   }
   println!("-------------------- String mut ref ");
   {
      fn main() {
          let mut s = String::from("hello");
      
          change1(&mut s);
          change2(&mut s);

          println!("{}",s);
      }
      
      fn change1(some_string: &mut String) {
          some_string.push_str(", world");
      }
      fn change2(some_string: &mut String) {
          some_string.push_str("  OK");
      }

      main();
   }
}
