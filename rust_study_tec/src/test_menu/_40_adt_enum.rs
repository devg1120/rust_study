
/*
 *   Algebraic type system in rust
 */

#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_variables)]

/*
#[derive(Debug)]
println!("{:#?}", foo);
*/
pub fn main() {


println!("--------------------  ");
{
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("{:?}", IpAddrKind::V4); // V4
    println!("{:?}", IpAddrKind::V6); // V6
}
main();
}

println!("--------------------  Enumは個別に型を持てる");
{
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("{:?}", IpAddrKind::V4(127, 0, 0, 1)); // V4(127, 0, 0, 1)
    println!("{:?}", IpAddrKind::V6(String::from("::1"))); // V6("::1")
}
main();
}
println!("--------------------  Enumはメソッドを定義できる");
{
#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

// メソッドを定義
impl IpAddrKind {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    IpAddrKind::V4(127, 0, 0, 1).call(); // V4(127, 0, 0, 1)
    IpAddrKind::V6(String::from("::1")).call(); // V6("::1")
}
main();
}
println!("--------------------  match");
{
enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let color = Color::Red;
    let color_jp = color_to_jp(color);

    println!("{}", color_jp); // あか
}

fn color_to_jp(color: Color) -> String {
    match color {
        Color::Red => String::from("あか"),
        Color::Blue => String::from("あお"),
        Color::Green => String::from("みどり"),
    }
}
main();
}
println!("--------------------  Enumに個別の型がある場合");
{
#[allow(dead_code)]
enum Color {
    Red(String),
    Blue,
    Green,
}

fn main() {
    let color = Color::Red(String::from("#ff0000"));
    let color_jp = color_to_jp(color);

    println!("{}", color_jp); // あか
}

fn color_to_jp(color: Color) -> String {
    match color {
        Color::Red(color_code) => {
            println!("{}", color_code); // #ff0000
            String::from("あか")
        }
        Color::Blue => String::from("あお"),
        Color::Green => String::from("みどり"),
    }
}
main();
}
println!("--------------------  Optionを使いこなそう");
/*
enum Option<T> {
    Some(T),
    None,
}
*/
{
fn main() {
    let some_value = Some(String::from("Hello")); // Someは値がある
    let none_value: Option<String> = None; // Noneは値が無い

    println!("{}", response(some_value)); // Hello World
    println!("{}", response(none_value)); // No value
}

fn response(value: Option<String>) -> String {
    match value {
        Some(value) => {
            value + " World"
        }
        None => String::from("No value"),
    }
}
main();
}
println!("-------------------- 代数的データ型 ");
/*
代数的データ型（Algebraic Data Type, ADT）とは、

複数の型を組み合わせることで定義されるデータ型のことです
関数型プログラミングや型システムにおいて、データの構造を明確に表現するために用いられます

代数的データ型は、直和型と直積型の2つの基本的な概念から構成されます

直積型 (Product Type):

 複数の型を組み合わせて、それら全ての値を保持する型です。
 例えば、{名前: 文字列, 年齢: 整数} のような構造は、文字列型と整数型を組み合わせた直積型の一例です。

直和型 (Sum Type):

 複数の型の中から、いずれか一つの値を保持する型です。
 例えば、結果 = 成功(値: 整数) | 失敗(エラーメッセージ: 文字列) のような型は
 成功時の整数値か、失敗時のエラーメッセージのいずれかを持つ直和型です。


代数的データ型のメリット:

 * 型安全なプログラミング:
    コンパイル時に型チェックを行うことで、実行時のエラーを減らし、
    より安全なコードを記述できます。Qiitaによると

 * データの構造を明確に表現:
     データの種類や構造を明確に定義できるため、コードの可読性や保守性が向上します
 * パターンマッチングとの相性:
     パターンマッチングと組み合わせることで、様々なケースに対応した処理を簡潔に記述できます。


代数的データ型  = 直和型 x 直積型 

    Ex =  T1  A  B   |
       :  T2         |  和 (１つ)
       :  T3  C      |
        
        ----> 積（組み合わせ）
*/
{
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quit");
        },
        Message::Move { x, y } => {
            println!("Move to x: {}, y: {}", x, y);
        },
        Message::Write(text) => {
            println!("Text message: {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change color to Red: {}, Green: {}, Blue: {}", r, g, b);
        },
    }
}
fn main() {

    process_message(Message::Move {x:100,y:200});
    process_message(Message::Write(String::from("abc-xyz 123456789")));
    process_message(Message::ChangeColor( 1,2,3));
    process_message(Message::Quit);

}
main();
}
println!("--------------------  Algebraic type system in rust is great");
{
#![allow(unused)]
type CheckNumber = u32;
type CardNumber = String;

enum CardType {
    Visa,
    Mastercard,
}

#[derive(Debug)]
struct CreditInfo(CheckNumber, CardNumber);

#[derive(Debug)]
enum PaymentMethod {
    Cash,
    Check(CheckNumber),
    Card(CreditInfo),
}

type PaymentAmount = f32;
#[derive(Debug)]
enum Currency {
    Eur,
    Usd,
}

#[derive(Debug)]
struct Payment {
    amount: PaymentAmount,
    currency: Currency,
    method: PaymentMethod,
}

trait PrintDetails {
    fn payment_info(&self) -> String;
}

impl PrintDetails for Payment {
    fn payment_info(&self) -> String {
        let method = match &self.method {
            PaymentMethod::Cash => String::from("cash"),
            PaymentMethod::Check(c) => format!("a check with number {:?}", c),
            PaymentMethod::Card(c) => format!("a check {} with a credit card {}", c.0, c.1),
        };

        format!(
            "An amount of {}, was paid in {:?}, using {}",
            &self.amount, &self.currency, method
        )
    }
}

fn main() {
    let cc_payment = Payment {
        amount: 100.33,
        currency: Currency::Usd,
        method: PaymentMethod::Card(CreditInfo(122, String::from("452389798712097"))),
    };

    let check_payment = Payment {
        amount: 0.33,
        currency: Currency::Eur,
        method: PaymentMethod::Check(42),
    };

    println!("Payment details \n {}", check_payment.payment_info());
    println!("Payment details \n {}", cc_payment.payment_info());
}
main();
}
println!("--------------------  `match`を使ったデータ抽出");
{
enum Message {
    Text(String),
    Image(String, u32, u32),
    Quit,
}

fn main() {
    let msg = Message::Image(String::from("photo.jpg"), 1280, 720);

    match msg {
        Message::Text(content) => println!("Text message: {}", content),
        Message::Image(path, width, height) => {
            println!("Image message: path={}, size={}x{}", path, width, height);
        }
        Message::Quit => println!("Quit message."),
    }
}
main();
}
println!("--------------------  `if let`を使ったシンプルな抽出");
{
enum Message {
    Text(String),
    Image(String, u32, u32),
    Quit,
}
fn main() {
    let msg = Message::Text(String::from("Hello, world!"));

    if let Message::Text(content) = msg {
        println!("Received a text message: {}", content);
    }
}
main();
}
println!("--------------------  ネストされたデータの処理");
{
enum NetworkEvent {
    Connected(String),
    Disconnected,
    Error(String),
}

enum AppEvent {
    Network(NetworkEvent),
    UserInput(String),
}

fn main() {
    let event = AppEvent::Network(NetworkEvent::Connected(String::from("192.168.0.1")));

    match event {
        AppEvent::Network(NetworkEvent::Connected(ip)) => println!("Connected to {}", ip),
        AppEvent::Network(NetworkEvent::Disconnected) => println!("Disconnected"),
        AppEvent::Network(NetworkEvent::Error(err)) => println!("Network error: {}", err),
        AppEvent::UserInput(input) => println!("User input: {}", input),
    }
}
main();
}
println!("--------------------  エラーハンドリング" );
{
enum AppError {
    NotFound(String),          // エラーに関連するリソース名
    PermissionDenied(String),  // アクセス拒否の理由
    Unknown,
}

fn get_user_data(user_id: u32) -> Result<String, AppError> {
    match user_id {
        1 => Ok(String::from("User data for user 1")),
        2 => Err(AppError::NotFound(String::from("User not found"))),
        3 => Err(AppError::PermissionDenied(String::from("Access denied"))),
        _ => Err(AppError::Unknown),
    }
}

fn main() {
    let result = get_user_data(2);

    match result {
        Ok(data) => println!("Success: {}", data),
        Err(AppError::NotFound(resource)) => println!("Error: {} not found.", resource),
        Err(AppError::PermissionDenied(reason)) => println!("Error: Permission denied ({})", reason),
        Err(AppError::Unknown) => println!("Error: Unknown error occurred."),
    }
}
main();
}
println!("--------------------  状態管理");
{
enum AppState {
    Initializing,
    Running { tasks_completed: u32 },
    Paused,
    Error(String),
}
fn transition_state(state: AppState) -> AppState {
    match state {
        AppState::Initializing => {
            println!("Transitioning from Initializing to Running.");
            AppState::Running { tasks_completed: 0 }
        }
        AppState::Running { tasks_completed } if tasks_completed >= 10 => {
            println!("All tasks completed. Transitioning to Paused.");
            AppState::Paused
        }
        AppState::Running { tasks_completed } => {
            println!("Completed task {}. Continuing Running.", tasks_completed + 1);
            AppState::Running { tasks_completed: tasks_completed + 1 }
        }
        AppState::Paused => {
            println!("Resuming from Paused.");
            AppState::Running { tasks_completed: 0 }
        }
        AppState::Error(err) => {
            println!("Error encountered: {}. Transitioning to Initializing.", err);
            AppState::Initializing
        }
    }
}
fn main() {
    let mut state = AppState::Initializing;

    for _ in 0..12 {
        state = transition_state(state);

        // 状態を確認する
        match &state {
            AppState::Initializing => println!("State: Initializing"),
            AppState::Running { tasks_completed } => {
                println!("State: Running (Tasks completed: {})", tasks_completed)
            }
            AppState::Paused => println!("State: Paused"),
            AppState::Error(err) => println!("State: Error ({})", err),
        }
    }
}
main();
}
println!("--------------------  ジェネリクスと列挙型");
{
enum Response<T> {
    Success(T),
    Error(String),
}
fn main() {
    let success_response: Response<i32> = Response::Success(200);
    let error_response: Response<&str> = Response::Error(String::from("Not Found"));

    match success_response {
        Response::Success(value) => println!("Success with value: {}", value),
        Response::Error(err) => println!("Error: {}", err),
    }

    match error_response {
        Response::Success(value) => println!("Success with value: {}", value),
        Response::Error(err) => println!("Error: {}", err),
    }
}
main();
}
println!("--------------------  ジェネリクスを活用したAPIレスポンス管理");
{
enum ApiResponse<T> {
    Ok(T),
    NotFound,
    Unauthorized,
    ServerError(String),
}

fn fetch_user_data(user_id: u32) -> ApiResponse<String> {
    match user_id {
        1 => ApiResponse::Ok(String::from("User data for user 1")),
        2 => ApiResponse::NotFound,
        _ => ApiResponse::ServerError(String::from("Internal Server Error")),
    }
}

fn main() {
    let response = fetch_user_data(1);

    match response {
        ApiResponse::Ok(data) => println!("Data received: {}", data),
        ApiResponse::NotFound => println!("User not found."),
        ApiResponse::Unauthorized => println!("Unauthorized access."),
        ApiResponse::ServerError(err) => println!("Server error: {}", err),
    }
}
main();
}


}


