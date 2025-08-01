/*
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Debug)]
*/

/*
option.unwrap()

 ↓

match option {
    Some(v) => v,
    None => panic!(...),
}
*/

pub fn main() {

    println!("-------------------- Option");
      let some_number = Some(5);
      let some_string = Some("a string");

      println!("{:#?}",some_number);
      println!("{:#?}",some_string);


}

