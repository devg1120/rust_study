
/*
    _23_mod
    ├── foo.rs
    ├── utils
    │   ├── nested.rs
    │   └── utility.rs
    └── utils.rs
*/

mod foo;
mod utils;

pub fn main() {
   println!("-------------------- mod");
   foo::foo_func();
   println!("-------------------- mod");
   utils::utility::test();

}
