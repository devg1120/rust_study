
pub fn main() {
        println!("-------------------- fn");
        fn add(x: i32, y: i32) -> i32 {
            return x + y;
        }

        println!("add={}", add(1,99));

        fn add2(x: i32, y: i32) -> i32 {
            x + y	// セミコロン(;)無し
        }
        println!("add2={}", add2(1,99));

}

