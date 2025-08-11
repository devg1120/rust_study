
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

let t = 2;
let mut c = 1;
if  c == t {
println!("--------------------  struct base di");
/*
struct ベースの DI
 サービスのインターフェイスを trait で定義し、 実装は struct で行う。
 struct はフィールドに依存関係を持ち、インスタンス生成時に実体を受け取る。
 Java や Ruby でいうクラスなら、コンストラクタで依存関係を受け取る感じ。
 依存を差し替えられるよう、フィールドの型にはジェネリクスを使う。
 単純でわかりやすい。
サンプル
 以下の例ではSvcBがSvcAに依存しています。 そのためSvcBを実装する型はSvcAを実装する型をフィールドに持ち、それを使います。
考慮事項
 依存が増えると型パラメータも増える。
 参照だけを持つ形だとライフタイムパラメータも必要になる。
 Rust には所有権があるので、複数のサービスに同じサービスのインスタンスを持たせようとすると面倒になりうる。
 シンプルではありますが、依存の数だけ型パラメータを持たなければいけないのは辛いですね。
*/
{
pub trait SvcA {
    fn a(&self) -> String;
}

pub trait SvcB {
    fn b(&self) -> String;
}

pub struct ImplA {}

impl SvcA for ImplA {
    fn a(&self) -> String {
        "impl-a".to_owned()
    }
}

pub struct ImplB<A: SvcA> {
    a: A,
}

impl<A: SvcA> SvcB for ImplB<A> {
    fn b(&self) -> String {
        format!("a: {}, b: {}", self.a.a(), "impl-b")
    }
}

fn main() {
    struct MockA {}
    impl SvcA for MockA {
        fn a(&self) -> String {
            "mock-a".to_owned()
        }
    }

    let b = ImplB { a: MockA {} };
    println!("{}", b.b() );
    assert_eq!(b.b(), "a: mock-a, b: impl-b");
}

pub fn use_b<B: SvcB>(b: B) -> String {
    println!("use_b ******");
    format!("[use] {}", b.b())
}
main();
}
};c += 1;

if  c == t {
println!("--------------------  trait base di");
/*
trait ベースの DI
 trait のデフォルト実装にロジックを置く。
 trait だとフィールドは使えないので、代わりに依存する trait を要求する。
 1つの struct に複数のサービス trait を集約して実装することもできる。
サンプル
 こちらはクラスベースの思考から離れ、オブジェクトというよりは型でロジックを分離する方法です。 以下は先程と同じSvcAとSvcBを trait ベースで再実装したものです。
考慮事項
 場合によってはこっちの方がシンプル。
 trait をimplするだけで実装が得られる。
 trait とそれを実装する struct を 1:1 で作らなくても良くなる。
 依存する trait をモック実装すればテストできる。
 1つの struct に実装を集約して同じインスタンスを使い回せば、所有権に悩まされずに済むかも。
 struct ベースの方法と違い、依存が増減する度に型パラメータを変更する必要がないので、 個人的にはこちらをメインに使う方が良さそうな気がしています。 ただこの方法では、1つの struct がたくさんのサービス trait を実装していく形にはなります (この例でいうHub)。 この調子で他のサービスも実装していくとHubがどんどん肥大化する感じはしますが、 実際にはそれをそのまま使うわけではなく、use_bでの使用例のようにその時必要な型としてだけ使うようにすれば、 その点はあまり問題なさそうに思えます。
*/
{
pub trait SvcA {
    fn a(&self) -> String {
        "svc-a".to_owned()
    }
}

// SvcB requires SvcA.
pub trait SvcB: SvcA {
    fn b(&self) -> String {
        format!("a: {}, b: {}", self.a(), "svc-b")
    }
}

pub struct Hub {}
impl SvcA for Hub {}
impl SvcB for Hub {}

fn main() {
    struct Mock {}
    impl SvcA for Mock {
        fn a(&self) -> String {
            "mock-a".to_owned()
        }
    }
    impl SvcB for Mock {}

    let b = Mock {};
    println!("{}", b.b() );
    assert_eq!(b.b(), "a: mock-a, b: svc-b");
}

pub fn use_b<B: SvcB>(b: B) -> String {
    format!("[use] {}", b.b())
}
main();
}
}; c += 1;


if  c == t {
println!("--------------------  trait base di muntl service");
/*
ちなみにもし複数のサービスが同名のメソッドを持っていても、呼び分けるのは簡単なので問題ありません。
*/

{
trait A1 {
    fn a(&self) -> i32 { 0 }
}
trait A2 {
    fn a(&self) -> i32 { 1 }
}

struct Hub {}
impl A1 for Hub {}
impl A2 for Hub {}

fn main() {
    let hub = Hub {};

    // hub.a() と同じ
    let a1 = A1::a(&hub);
    let a2 = A2::a(&hub);
    println!("{}, {}", a1, a2);
}
main();
}
}; c += 1;

/*
println!("--------------------  not op");
{
 
// インターフェイス
pub trait IsSvcA {
    fn a(&self) -> String;
}

// 依存関係
pub trait SvcA {}

// インターフェイス
pub trait IsSvcB {
    fn b(&self) -> String;
}

// 依存関係
pub trait SvcB: IsSvcA {}

impl<T: SvcA> IsSvcA for T {
    fn a(&self) -> String {
        "svc-a".to_owned()
    }
}

impl<T: SvcB> IsSvcB for T {
    fn b(&self) -> String {
        format!("a: {}, b: {}", self.a(), "svc-b")
    }
}

fn main() {
    struct Mock {}
    // COMPILE ERROR
    impl SvcB for Mock {
        fn b(&self) -> String {
            "mock-b".to_owned()
        }
    }

    assert_eq!(use_b(Mock {}), "[use] mock-b");

}
pub fn use_b<B: IsSvcB>(b: B) -> String {
    format!("[use] {}", b.b());
}
main();
}
*/

//println!("--------------------  static-constructor-di");
println!("--------------------  dynamic-constructor-di");
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
println!("--------------------  ");
{

}
println!("--------------------  ");
{

}
}

