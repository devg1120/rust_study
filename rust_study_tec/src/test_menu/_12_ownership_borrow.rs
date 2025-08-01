
/*
 *
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

pub fn main() {


println!("--------------------  所有権を返す");

{
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // 所有権は外に移動
}

fn main() {
    let foo = do_something();
    // foo は所有者になる
    // 関数のスコープの終端により、foo はドロップ
    println!("{:#?}", foo);
}
main();
}

println!("--------------------  参照による所有権の借用");
{
struct Foo {
    x: i32,
}

fn main() {
    let foo = Foo { x: 42 };
    let f = &foo;               //参照
    println!("{}", f.x);
    // f はここでドロップ
    // foo はここでドロップ
}
main();
}
println!("--------------------  参照による所有権の可変な借用");
{
struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    let f = &mut foo;

    // 失敗: do_something(foo) はここでエラー
    // foo は可変に借用されており移動できないため

    // 失敗: foo.x = 13; はここでエラー
    // foo は可変に借用されている間は変更できないため

    f.x = 13;
    // f はここから先では使用されないため、ここでドロップ
    
    println!("{}", foo.x);
    
    // 可変な借用はドロップされているため変更可能
    foo.x = 7;
    
    // foo の所有権を関数に移動
    do_something(foo);
}
main();
}
println!("--------------------  参照外し");
{
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // 所有者の値を取得
    *f = 13;      // 参照の所有者の値を設定
    println!("{}", bar);
    println!("{}", foo)
}
println!("--------------------  借用したデータの受け渡し");
/*
可変な参照が 1 つだけか、不変な参照が複数かのどちらかが許可されます。両方を同時には使用できません。
参照は所有者よりも長く存在してはなりません。
*/
{
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something(f: &mut Foo) {
    f.x += 1;
    // f への可変な参照はここでドロップ
}

fn main() {
    let mut foo = Foo { x: 42 };
    do_something(&mut foo);
    // 関数 do_something で可変な参照はドロップされるため、
    // 別の参照を作ることが可能
    do_something(&mut foo);
    println!("{:#?}", foo);
    // foo はここでドロップ
}
main();
}
println!("--------------------  参照の参照");
{
#[derive(Debug)]
struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x はここでドロップされるため、不変な参照が作成可能
    let y = do_something(&foo);
    println!("{}", y);
    println!("{:#?}", foo);
    // y はここでドロップ
    // foo はここでドロップ
}
main();
}
println!("--------------------  明示的なライフタイム");
{
#[derive(Debug)]
struct Foo {
    x: i32,
}

// 引数 foo と戻り値はライフタイムを共有
fn do_something<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x はここでドロップされるため、不変な参照が作成可能
    let y = do_something(&foo);
    println!("{}", y);
    println!("{:#?}", foo);
    // y はここでドロップ
    // foo はここでドロップ
}
main();
}

println!("--------------------  複数のライフタイム");
{
#[derive(Debug)]
struct Foo {
    x: i32,
}

// foo_b と戻り値はライフタイムを共有
// foo_a のライフタイムは別
fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    // ここから先は foo_b のライフタイムしか存在しないため、
    // foo_a はここでドロップ
    println!("{}", x);
    // x はここでドロップ
    // foo_b はここでドロップ
}
main();
}
println!("--------------------  スタティックライフタイム");
{
static PI: f64 = 3.1415;

fn main() {
    // スタティック変数は関数スコープでも定義可能
    static mut SECRET: &'static str = "swordfish";

    // 文字列リテラルは 'static ライフタイム
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);
/*
    // ルールを破ることはできますが、それを明示する必要があります。
    unsafe {
        // 文字列リテラルは 'static なので SECRET に代入可能
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
*/
}
main();
}
println!("--------------------  データ型のライフタイム");
{
#[derive(Debug)]
struct Foo<'a> {
    i:&'a i32
}

fn main() {
    let x = 42;
    let foo = Foo {
        i: &x
    };
    println!("{}",foo.i);
    println!("{:#?}", foo);
}
main();
}
println!("--------------------  ");
{

}




}

