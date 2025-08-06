
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
//use tower::BoxError;
use http::Response;

use axum::extract::Path;
use std::collections::HashMap;
use axum::response::IntoResponse;
//use axum_core::response::IntoResponse;
use axum::RequestPartsExt;
//use axum::Json;

#[tokio::main]
async fn main() {

    let app = Router::new().route("/", get(root_handler))
                           .route("/ping", post(ping))
                           .route("/users/:user_id/posts/:post_id", get(user_post))
                           .route("/users/:user_id", post(user_message))
                           .route("/users2/:user_id/:name", post(user_message2));

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


/**************************************/
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


/**************************************/
#[derive(Deserialize)]
struct Params {
    //user_id: u64,
    user_id: String,
    name: String,
}


#[async_trait]
//impl<B> extract::FromRequest<B> for Params
impl<B> extract::FromRequestParts<B> for Params
where
        B: Send + Sync,
{
    type Rejection = Response<Body>;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &B,
        ) -> Result<Self, Self::Rejection> {

       let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

      for key in params.keys() {
         println!("{}", key);

      }
       let user_id = params
            .get("user_id")
            .unwrap();

       let name = params
            .get("name")
            .unwrap();


        //let  name = "GUSA";

        Ok(Params {
            user_id: user_id.clone(),
            name: name.clone(),
        })
    }
}


#[derive(Deserialize)]
struct JsonData {
    name: String,
}


#[derive(Serialize)]
struct Message2 {
    message: String,
}

async fn user_message2(
    params: Params,
    extract::Json(j_name): extract::Json<Name>
) -> response::Json<Message2> {
    let p_user_id = params.user_id;
    let p_name = params.name;
    let j_name = j_name.name;
    response::Json(Message2 {
        message: format!("Hello {}, your id is {}: {}", j_name, p_user_id , p_name),
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
$ curl -H 'Content-Type: application/json' -d '{{\"name\": \"techno\"}}' http://localhost:3000/users2/99/gusa
{{\"message\":\"Hello gusa, your id is 1\"}}

");

main();

}

}

