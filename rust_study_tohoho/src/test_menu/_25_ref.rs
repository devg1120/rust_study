#[allow(dead_code)]
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn main() {
   println!("-------------------- reference");
   let a = 123;
   let p = &a;		// 123という値が格納された領域への参照をpに代入する
   println!("{}", *p);	// pが参照する領域の値(123)を出力する
   let ref p2 = a;
   println!("{}", *p2);	// => 123
                       
   println!("-------------------- reference mut / dereference");
   let mut a = 123;	// ミュータブルな変数aを定義
   let p = &mut a;	// ミュータブルな参照pを定義
   *p = 456;		// 参照先の値を456に書き換える
   println!("{}", a);	// => 456
                        //
   println!("-------------------- copy trait");
   {
    let a = 10;            // immutable object
    let b = a;             // copy
    println!("{} {}", a, b); // borrow check!! - OK
   }
   {
    let a = 10;                  // immutable object
    let a_ref = &a;              // reference
    let a_ref_copy = a_ref;      // copy reference
    println!("{} {} {}", a, a_ref, a_ref_copy); // borrow check!! - OK
   }
   {
    let mut a = 10;                 // mutable object
    let a_mut_ref = &mut a;         // mutable reference
    #[allow(unused_variables)]
    let a_mut_ref_move = a_mut_ref; // move mutable reference
    //println!("{}", a_mut_ref);        // borrow check!! - Error!
   }
   {
    let mut a = 10;                 // mutable object
    let a_mut_ref = &mut a;         // mutable reference
    let a_mut_ref_move = a_mut_ref; // move mutable reference
    println!("{}", a_mut_ref_move);   // borrow check!! - OK
   }
                           
}
