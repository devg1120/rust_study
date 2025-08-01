#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn main() {
        println!("-------------------- if ");
        let n = 2;
        if n == 1 {
            println!("One");
        } else if n == 2 {
            println!("Two");
        } else {
            println!("Other");
        }
        println!("-------------------- while ");
        let mut n = 0;
        while n < 10 {
            n += 1;
            println!("{}",n);
        }
        println!("-------------------- for ");
        for i in 0..10 {
            println!("{}", i);
        }
        println!("-------------------- loop ");
        let mut n = 0;
        loop {
            n += 1;
            if n == 10 {
                break;
            }
            println!("{}",n);
        }
        println!("-------------------- break / continue ");
        let mut n = 0;
        loop {
            n += 1;
            if n == 2 {
                continue;
            }
            if n == 8 {
                break;
            }
            println!("{}", n);
        }
        println!("-------------------- match ");
        let x = 2;
        match x {
            1 => println!("One"),
            2 => println!("Two"),
            3 => println!("Three"),
            _ => println!("More"),
        }

}

