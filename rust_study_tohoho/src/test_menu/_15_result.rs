/*
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Debug)]
*/

/*
 
result.unwrap()

 ↓

match result {
    Ok(v) => v,
    Err(e) => panic!(...),
}

*/
pub fn main() {

    println!("-------------------- Result");

       fn positive(i:i64) -> Result<u64, String> {
         if i >= 0 {
           Ok(i as u64)
         } else {
           Err(String::from("負の整数です。"))
         }
       }
       
       let p = positive(9);
       match p {
           Ok(num) => { println!("正の整数{}です。",num); }
           Err(err) => { println!("{}",err); }
       }

    println!("-------------------- Result");
    {
         use std::{fs::File, io::ErrorKind};
         let f = File::open("hello.txt");
          let _f = match f {
              Ok(file) => {
                  println!("file open");
                  file
              },
              Err(ref error) if error.kind() == ErrorKind::NotFound => {
                  println!("file create");
                  match File::create("hello.txt") {
                      Ok(fc) => fc,
                      Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
                  }
              },
              Err(error) => {
                  panic!("There was a problem opening the file: {:?}", error)
              },
          };
    }


}

