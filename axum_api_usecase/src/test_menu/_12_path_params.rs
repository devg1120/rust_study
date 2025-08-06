
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

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(root_handler))
                           .route("/ping", post(ping))
                           .route("/users/:user_id/posts/:post_id", get(user_post));

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

println!("
$ curl http://localhost:3000/
 Hello, World!
$ curl -H 'Content-Type: application/json' -d ' {{\"count\": 0}}' http://localhost:3000/ping
 {{\"count\": 1}}
$ curl -H 'Content-Type: application/json' -d ' {{\"count\": 3}}' http://localhost:3000/ping
 {{\"count\": 4}}
$ curl http://localhost:3000/users/1/posts/2
user_id: 1, post_id: 2
");

main();

}

}

