
struct Point {
    x: i32,
    y: i32,
}

union MyUnion {
    f1: u32,
    f2: u32,
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
}

#[allow(unused_assignments)]
pub fn main() {

    println!("--------------------  struct");
    let p = Point { x: 100, y: 200 };
    println!("{} {}", p.x, p.y);

    println!("--------------------  union");
    let u = MyUnion { f1: 123 };
    unsafe {
        println!("{}", u.f1);
        println!("{}", u.f2);	// メモリを共用しているのでこちらも123と表示される
    }
    println!("--------------------  enum");
    let color = Color::Red;
    match color {
       Color::Red =>  println!("Red"),
       Color::Green =>  println!("Greem"),
       Color::Blue =>  println!("Blue"),
    }
    println!("--------------------  tupple");
    let tup = (10, "20", 30);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    println!("--------------------  array");
    let arr = [10, 20, 30];
    println!("{} {} {}", arr[0], arr[1], arr[2]);
    
    for v in &arr {
        println!("{}", v);
    }
    for i in 0..arr.len() {
        println!("{} -> {}", i, arr[i]);
    }
    println!("--------------------  vec");
    let mut vect = vec![10, 20, 30];
    vect.push(40);
    println!("{} {} {} {}", vect[0], vect[1], vect[2], vect[3]);
    
    for v in &vect {
        println!("{}", v);
    }
    println!("--------------------  HashMap");
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("x", 10);
    map.insert("y", 20);
    map.insert("z", 30);
    println!("{} {} {}", map["x"], map["y"], map["z"]);
    
    for (k, v) in &map {
        println!("{} {}", k, v);
    }
    println!("--------------------  &str");
        let mut name1: &str = "Yamada";
        name1 = "Tanaka";
        println!("{} ",name1);

    println!("--------------------  String");
        let mut name2 = String::from("Yamada");
        name2 = "Tanaka".to_string();
        name2.push_str(" Taro");
        println!("{} ",name2);

    println!("-------------------- heep Box");
       struct Point2 {
           x: i32,
           y: i32,
       }
       
       impl Drop for Point2 {
           fn drop(&mut self) {	// Pointが解放される際に呼び出される
               println!("Bye!");
           }
       }
       let p: Box<Point2> = Box::new(Point2 { x: 100, y: 200 });
       println!("{} {}", p.x, p.y);

    println!("-------------------- slice");

      let s = String::from("ABCDEFGH");
      let s1 = &s[0..3];		// 0番目から3番目の手前までのスライス("ABC")
      let s2 = &s[3..6];		// 3番目から6番目の手前までのスライス("DEF")
      println!("{} {}", s1, s2);	// => ABC DEF
      
      let a = [10, 20, 30, 40, 50, 60, 70, 80];
      let a1 = &a[0..3];		// 0番目から3番目の手前までのスライス[10, 20, 30]
      let a2 = &a[3..6];		// 0番目から3番目の手前までのスライス[40, 50, 60]
      println!("{:?} {:?}", a1, a2);// => [10, 20, 30] [40, 50, 60]
                                    //
    println!("-------------------- type ailas");

     type Meter = u32;
     type Millimeter = u32;
     let m: Meter = 12;
     let mm: Millimeter = 12000;
     println!("{} {}", m, mm);

    println!("-------------------- typeof");
     fn type_of<T>(_: T) -> &'static str {
         std::any::type_name::<T>()
     }

     let x = 123;
     println!("{}", type_of(x));
}

