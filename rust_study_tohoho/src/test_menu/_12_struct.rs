
#[allow(unused_assignments)]
#[allow(dead_code)]
#[allow(unused_variables)]

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn main() {

    println!("--------------------  struct");
    #[allow(unused_variables)]
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    #[allow(unused_variables)]
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("anotheremail@example.com");
    
    println!("{:#?}", user1);
    println!("{:#?}", user2);

    println!("--------------------  tuple struct");
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(1, 2, 3);
    let Point (x,y,z) = Point(9, 8, 7);
    
    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", x, y, z);

    println!("--------------------  fmt::Display");
    {
        use std::fmt;
        
        struct Password(String);
        
        impl fmt::Display for Password {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0.chars().map(|_| '*').collect::<String>())
            }
        }
        
        let a = String::from("123456789");
        println!("{}", a); // 123456789
            
        let a = Password(String::from("123456789"));
        println!("{}", a); // *********
     }
    println!("--------------------  impl method");
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }

        let rect1 = Rectangle { width: 30, height: 50, };
        println!("The area of the rectangle is {} square pixels.", rect1.area());

    }
}

