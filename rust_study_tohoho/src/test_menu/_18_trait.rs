#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
/*
 * trait は特質の意味で、構造体が実装すべきメソッドを定義します。
 * 他言語の インタフェース(interface) に似ています
 * 。例えば、std::fmt::Display トレイトを実装した構造体は println!() の "{}" で
 * 、std::fmt::Debug トレイトを実装した構造体は "{:?}" で書き出すことが可能です。
*/
pub fn main() {
        println!("-------------------- trait 1");
        struct Rect { width: u32, height: u32 }
        
        trait Printable { fn print(&self); }
        
        impl Printable for Rect {
            fn print(&self) {
                println!("width:{}, height:{}", self.width, self.height)
            }
        }

        let r = Rect { width: 200, height: 300 };
        r.print();
       
        println!("-------------------- trait 2");

        struct Rect2<T> { width: T, height: T, }
        
        trait Printable2 { fn print(&self); }
        impl<T> Printable2 for Rect2<T> where T: std::fmt::Display {
            fn print(self: &Rect2<T>) {
                println!("{}x{}", self.width, self.height);
            }
        }

        let r1: Rect2<i32> = Rect2{ width: 100, height: 200 };
        let r2: Rect2<i64> = Rect2{ width: 100, height: 200 };
        r1.print();
        r2.print();

        println!("-------------------- trait 3");
        use std::boxed::Box;
        
        struct Dog {}
        struct Cat {}
        trait Animal { fn cry(&self); }
        impl Animal for Dog { fn cry(&self) { println!("Bow-wow"); } }
        impl Animal for Cat { fn cry(&self) { println!("Miaow"); } }
        
        fn get_animal(animal_type: &str) -> Box<dyn Animal> {
            if animal_type == "dog" {
                return Box::new(Dog {});
            } else {
                return Box::new(Cat {});
            }
        }
        
        get_animal("dog").cry();
        get_animal("cat").cry();

        println!("-------------------- trait 4");
        {
          pub trait Geometry {
              fn area(&self) -> f64;
              fn name(&self) -> &str { return "Geometry" }
          }
          
          struct Rectangle { width: u32, height: u32 }
          
          impl Geometry for Rectangle {
              fn area(&self) -> f64 {
                  self.width as f64 * self.height as f64
              }
              fn name(&self) -> &str { return "Rectangle" }
          }
          
          struct Triangle { bottom: u32, height: u32 }
          
          impl Geometry for Triangle {
              fn area(&self) -> f64 {
                  self.bottom as f64 * self.height as f64 * 0.5
              }
              fn name(&self) -> &str { return "Triangle" }
          }

          let a = Rectangle { width: 10, height: 20 };
          let b = Triangle  { bottom: 20, height: 5 };
          println!("{} area={}", a.name(), a.area());
          println!("{} area={}", b.name(), b.area());
        }
 }
