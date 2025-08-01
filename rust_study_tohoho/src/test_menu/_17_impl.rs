#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
/*
 * Rust ではクラスはサポートされていませんが、
 * impl によって構造体にメソッドを加えることができます
 * self は自オブジェクトを示します
*/
pub fn main() {
        println!("-------------------- impl ");
        struct Rect { width: u32, height: u32 }
        
        impl Rect {
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }
        let r = Rect { width: 200, height: 300 };
        println!("{}", r.area());

}

