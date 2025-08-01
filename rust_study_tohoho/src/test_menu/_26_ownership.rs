#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}


pub fn main() {
   println!("-------------------- ownership move");
   
   fn func1() {
       let name = String::from("ABC");
       println!("{}", name);
       func2(name);			// ここで所有権がfunc2()のnameに移動してしまう
       //println!("{}", name);   	// func2()終了時に開放済の領域を参照しているのでエラー
   }
   
   fn func2(name: String) {		// func1()から所有権を奪い取る
       println!("{}", name);
   }					// この時点でヒープ領域が解放されてしまう
                                      
   func1();

   println!("-------------------- ownership move return");
   fn func3() {
       let mut name = String::from("ABC");
       println!("{}", name);
       name = func4(name);		// 所有権を渡した後、返却してもらう
       println!("{}", name);	
   }
   
   fn func4(mut name: String)-> String {	
       println!("{}", name);
       name = String::from("xyz");
       return name                   // 所有権を渡した後、返却してもらう
   }				

   func3();

   println!("-------------------- ownership & borrowing");
   fn func11() {
       let name = String::from("ABC");
       println!("{}", name);
       func12(&name);			// 参照のみを渡して所有権は渡さない
       println!("{}", name);		// 所有権が残っているので参照可能
   }
   
   fn func12(name: &String) {		// func1()から参照のみを借用する
       println!("{}", name);
   }					// 参照のみなのでヒープ領域は解放されない
                                        //

   func11();

   println!("-------------------- ownership &mut borrowing");
   fn func21() {
       let mut name = String::from("ABC");
       println!("{}", name);
       func22(&mut name);			// 参照のみを渡して所有権は渡さない
       println!("{}", name);		// 所有権が残っているので参照可能
   }
   
   fn func22( name: &mut String) {		// func1()から参照のみを借用する
       println!("{}", name);
       *name = String::from("xyz");
   
   }					// 参照のみなのでヒープ領域は解放されない
                                        //
   func21();






}
