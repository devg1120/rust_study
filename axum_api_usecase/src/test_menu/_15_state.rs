
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


println!("--------------------  ");
{
use axum::{routing::get, routing::post, Router};
use axum::{extract, response};
use serde::{Serialize, Deserialize};

use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::add_extension::AddExtensionLayer;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State { counter: 0 }));
    let app = Router::new().route("/", get(root_handler))
                           .route("/ping", post(ping))
                           .route("/users/:user_id/posts/:post_id", get(user_post))
                           .route("/users/:user_id", post(user_message))
                           .route("/increment", get(increment))
                           .layer(AddExtensionLayer::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn root_handler() -> &'static str {
    "Hello, World!\n"
}

#[derive(Deserialize)]
struct Ping {
    count: i64,
}

#[derive(Serialize)]
struct Pong {
    count: i64,
}


async fn ping(extract::Json(ping): extract::Json<Ping>) -> response::Json<Pong> {
    response::Json(Pong {
        count: ping.count + 1,
    })
}


//async fn user_post(extract::UrlParams(params): extract::UrlParams<(u64, u64)>) -> String {
async fn user_post(extract::Path(params): extract::Path<(u64, u64)>) -> String {
    let user_id = params.0;
    let post_id = params.1;
    format!("user_id: {}, post_id: {}\n", user_id, post_id)
}


#[derive(Deserialize)]
struct Name {
    name: String,
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn user_message(
    extract::Path(params): extract::Path<(u64,)>,
    extract::Json(name): extract::Json<Name>,
) -> response::Json<Message> {
    let user_id = params.0;
    let name = name.name;
    response::Json(Message {
        message: format!("Hello {}, your id is {}", name, user_id),
    })
}

#[derive(Serialize, Deserialize)]
struct State {
    counter: i64,
}

type SharedState = Arc<Mutex<State>>;

async fn increment(extract::Extension(state): extract::Extension<SharedState>) -> response::Json<State> {
    let mut s = state.lock().await;
    s.counter += 1;
    let current = s.counter;
    response::Json(State { counter: current })
}

println!("
$ curl http://localhost:3000/
 Hello, World!
$ curl -H 'Content-Type: application/json' -d ' {{\"count\": 0}}' http://localhost:3000/ping
 {{\"count\": 1}}
$ curl -H 'Content-Type: application/json' -d ' {{\"count\": 3}}' http://localhost:3000/ping
 {{\"count\": 4}}
$ curl http://localhost:3000/users/1/posts/2
user_id: 1, post_id: 2
$ curl -H 'Content-Type: application/json' -d '{{\"name\": \"techno\"}}' http://localhost:3000/users/1
{{\"message\":\"Hello techno, your id is 1\"}}
curl http://localhost:3000/increment
{{\"counter\":1}}
");

main();

}

}

