


pub fn main() {
    /*
   println!("-------------------- mod");
   foo::foo_func();
   println!("-------------------- mod");
   utils::utility::test();
    */
   println!("-------------------- lib");
   rust_study::run(); 
   rust_study::module_a::run(); 
   rust_study::module_b::run(); 
   rust_study::module_a::sub_module::run(); 
   rust_study::module_b::sub_module::run(); 

   println!("\n-------------------- lib domain");

      /* 
      domain
      ├── entity
      │   └── node.rs
      ├── entity.rs
      ├── value_object
      │   └── id.rs
      └── value_object.rs
      */

   let tree = r#"
      domain
      │
      ├── entity.rs
      ├── entity
      │   └── node.rs
      │
      ├── value_objeca.rs
      └── value_object
           └── id.rs
   "#;

   println!("{}",tree);

   use rust_study::domain::{entity::node::Node, value_object::id::ID};

   let node = Node::new(ID::new("1"), "Node 1");
   println!("Hello, module: {:?}", node);

}
