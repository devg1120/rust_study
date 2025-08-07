
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

/*
#[derive(Debug)]
println!("{:#?}", foo);
*/
pub fn main() {


println!("--------------------  derive");
/*
下はderive可能なトレイトの一覧です。

型の比較に関連するトレイト: Eq, PartialEq, Ord, PartialOrd.
Clone：これはコピーによって&TからTを作成するトレイト
Copy：これはムーブセマンティクスの代わりにコピーセマンティクスにするためのトレイト
Hash：これは&Tからハッシュ値を計算するためのトレイト
Default：これは空っぽのインスタンスを作成するためのトレイト
Debug：これは{:?}フォーマッタを利用して値をフォーマットするためのトレイト

*/
{
// `Centimeters`は比較可能なタプルになります。
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`は出力可能なタプルになります。
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`には特にアトリビュートを付け加えません。
struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // エラー：`Debug`トレイトを実装していないため`Seconds`は出力できません。
    //println!("One second looks like: {:?}", _one_second);
    // TODO ^ この行をアンコメントしてみましょう。

    // エラー`PartialEq`トレイトを実装していないため`Seconds`は比較できません。
    //let _this_is_true = (_one_second == _one_second);
    // TODO ^ この行をアンコメントしてみましょう。

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
main();
}

println!("--------------------  Drop");
{
struct Droppable {
    name: &'static str,
}

// このちょっとした実装で、`drop`にコンソール出力機能がつきます。
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // `drop`関数を用いて変数を手動で開放することもできます。
    drop(_a);
    // TODO ^ この行をコメントアウトしてみましょう。

    println!("end of the main function");

    // `_a`はここで`drop`されることは *ありません* 。なぜならば、上ですでに
    // （手動で）`drop`されているためです。
}
main();
}
println!("--------------------  Iteratorトレイト");
{
struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element,
// and an `associated type` to declare the return type of the iterator.
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    // ここではイテレーションの流れを`.curr`と`.next`を使用して定義しています。
    // 返り値の型は`Option<T>`で、これは：
    //     * `Iterator`が終了した時は`None`を返します。
    //     * そうでなければ`Some`でラップされた値を返します。
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // フィボナッチ数列には終端がないので、`Iterator`は決して
        // `None`を返さず、常に`Some`が返されます。
        Some(current)
    }
}

// フィボナッチ数列のジェネレータを返します。
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    // `0..3`は0, 1, 2をジェネレートする`Iterator`。
    let mut sequence = 0..3;

    println!("Four consecutive `next` calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    // `for`は`None`を返すまで、イテレータを舐めていき、出てきた`Some`を
    // アンラップして変数（ここでは`i`）に束縛します。
    println!("Iterate through 0..3 using `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // `take(n)`メソッドは`Iterator`を先頭から`n`番目の要素までに減らします。
    println!("The first four terms of the Fibonacci sequence are: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // `skip(n)`メソッドは`Iterator`の先頭から`n`番目までの要素をとばします。
    println!("The next four terms of the Fibonacci sequence are: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // `iter`メソッドは配列やスライスからイテレータを作成します。
    println!("Iterate the following array {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
main();
}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
}

