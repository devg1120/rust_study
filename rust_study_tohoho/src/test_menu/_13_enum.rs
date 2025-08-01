
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

    println!("--------------------  enum 1");
    #[derive(Debug)]
    #[allow(dead_code)]
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:#?}", four);
    println!("{:#?}", six);
    
    
    println!("--------------------  enum 2");
    #[derive(Debug)]
    #[allow(dead_code)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("{:#?}", home);
    println!("{:#?}", loopback);


}

