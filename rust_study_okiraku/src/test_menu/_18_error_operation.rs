
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {
/*
Rust のエラー処理は Option や Result を使ってエラーを返すことが基本
マクロ panic! はエラーメッセージを表示してプログラムを終了する
Rust ではこれを「パニック (panic)」という
パニックの種類によっては std::panic::catch_unwind() で捕捉することもできるようだ
Option と Result のメソッド unwrap() は Some(T) や Ok(T) から T を取り出す
None や Err(E) の場合、unwrap() はパニックする
メソッド unwrap_or(default) を使うとパニックせずに default を返すことができる
Option のまま Some の値を処理したい場合はメソッド map() を使う
map() は引数の関数に Some の値を渡して実行し、その値を Some に格納して返す
None の場合は何もせずに None を返す
map() に渡す関数が Option を返す場合、返り値は Option の入れ子になる
これを一重の Option にしたい場合はメソッド and_then() を使う
繰り返しになるが、and_then() に渡す関数は Option を返すことに注意
and_then() は関数の返り値をそのまま返すだけ
関数型言語の flatmap() と同じ
Result にも map() や and_then() がある
Err(E) に作用する map_err() や or_else() もある
Option には Result に変換するメソッド ok_or(err) がある
None の場合、引数 err が Reuslt の Err(E) にセットされる

*/

println!("--------------------   Option, Result と map() の使用例");
{
    let a = [1, 2, 3, 4, 5];
    let r1 = a.iter().find(|&x| x % 3 == 0);
    println!("{}", r1.map(|&x| x * 10).unwrap());
    let r2 = a.iter().find(|&x| x % 6 == 0);
    println!("{}", r2.map(|&x| x * 10).unwrap_or(0));
    println!("{}", "12345".parse::<i32>().map(|x| x * 2).unwrap());
    println!("{}", "abcde".parse::<i32>().map(|x| x * 2).unwrap_or(0))
}

println!("--------------------  後置演算子 ? の使用例");
/*
後置演算子 ? を使うと、Err(E) が返された時点でリターンする
この時の返り値は Err(E)
Ok(T) が返された場合は、そこから T を取り出す
Rust ではこれを「早期リターン」という
返り値のデータ型は Result でないとコンパイルエラー
*/
{
fn parse_buff(buff: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut r: Vec<i32> = vec![];
    for s in buff {
        let n = s.parse()?;
        r.push(n)
    }
    Ok(r)
}

fn main() {
    let s0 = ["123", "456", "789"];
    println!("{:?}", parse_buff(&s0));
    let s1 = ["123", "abc", "789"];
    println!("{:?}", parse_buff(&s1));
}
main();
}
}

