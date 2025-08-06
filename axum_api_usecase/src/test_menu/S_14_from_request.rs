
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
use axum::async_trait;
use axum::body::Body;
use tower::BoxError;
use http::Response;

// add
/*
use axum::{
    async_trait,
    extract::{self, FromRequest, RequestParts},
    prelude::*,
    response::{self, IntoResponse},
};
use http::Response;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower::BoxError;
*/
#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(root_handler))
                           .route("/ping", post(ping))
                           .route("/users/:user_id/posts/:post_id", get(user_post))
                           .route("/users/:user_id", post(user_message));

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

/*
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
*/

/**************************************/
#[derive(Deserialize)]
struct Params {
    user_id: u64,
    name: String,
}

#[async_trait]
impl<B> extract::FromRequest<B> for Params
where
    B: Send + http_body::Body,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Response<Body>;

    async fn from_request(req: &mut impl  extract::FromRequestParts<B>) -> Result<Self, Self::Rejection> {

        //let url_params = extract::UrlParamsMap::from_request(req)
        let url_params = extract::Path::from_request(req)
            .await
            .map_err(response::IntoResponse::into_response)?;
        let user_id = url_params
            .get_typed("user_id")
            .unwrap()
            .unwrap();

        let json_params: extract::Json<serde_json::Value> = extract::Json::from_request(req)
            .await
            .map_err(response::IntoResponse::into_response)?;
        let name = json_params.0.get("name")
            .unwrap()
            .as_str()
            .unwrap();

        Ok(Params {
            user_id: user_id,
            name: name.to_string(),
        })
    }
}

#[derive(Serialize)]
struct Message {
    message: String,
}

async fn user_message(
    params: Params
) -> response::Json<Message> {
    let user_id = params.user_id;
    let name = params.name;
    response::Json(Message {
        message: format!("Hello {}, your id is {}", user_id, name),
    })
}
/**************************************/

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

");

main();

}

}

