fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
pub fn main() {
        println!("-------------------- closure ");
        let square = | x: i32 | {
            x * x
        };
        println!("{}", square(9));
        println!("{}", type_of(&square));

        println!("-------------------- closure move");

        let msg = String::from("Hello");	// クロージャー外変数msg
        let func = move |x: &str| {		// 所有権をクロージャーに移動すること宣言
            println!("{}", msg + x);		// 参照したクロージャー外変数の所有権はクロージャーに移動
        };					// クロージャー終了時に所有者が不在となり解放される
        func("+OK");				// クロージャーを呼び出す
                                        
        println!("-------------------- closure return function");
        fn get_closure(s : i32) -> impl Fn(i32) -> i32 {
            let ss: i32 = s;
            let square = move | x: i32 | {
                x * x + ss
             };
            return square;

        }

        let cl1 = get_closure(1000);
        let cl2 = get_closure(3000);
        println!("cl1 {}", cl1(9));
        println!("cl2 {}", cl2(9));
        println!("cl1 {}", cl1(3));
        println!("cl2 {}", cl2(3));

}

