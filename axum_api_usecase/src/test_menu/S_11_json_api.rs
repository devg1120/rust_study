
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


println!("--------------------  json api");
{
use axum::{
    extract,
    response,
    prelude::*,
};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = route("/ping", post(ping));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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

println!(r#"
$ curl -H 'Content-Type: application/json' -d '{\"count\": 0}' http://localhost:3000/ping
{"count":1}
$ curl -H 'Content-Type: application/json' -d '{\"count\": 3}' http://localhost:3000/ping
{"count":4}
"#);

main();

}

}

