#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn main() {
   println!("-------------------- thread");
   use std::thread;
   use std::time::Duration;

   // スレッドを起動する
   // 引数にクロージャー(ラムダ関数)を指定
   let th = thread::spawn(|| {
        for _i in 1..10 {
            println!("A");
            thread::sleep(Duration::from_millis(100));
        }
   });
   th.join().unwrap();
   println!("Finished");

   println!("-------------------- thread move");
   let str = String::from("ABC");
   let th = thread::spawn(move || {	// 所有権を引き渡すことを明示
        for _i in 1..10 {
            println!("{}", str);	// strの所有権を得る
            thread::sleep(Duration::from_millis(100));
        }
   });
   th.join().unwrap();
   println!("Finished");
}
