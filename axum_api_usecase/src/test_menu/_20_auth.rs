
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

use std::{collections::HashMap, sync::Arc};

use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    middleware::Next,
    response::Response,
    RequestExt as _,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use http::{request::Parts, StatusCode};

pub type Token = String;
pub type UserMap = HashMap<Token, User>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub username: String,
}

pub fn build_user_map() -> UserMap {
    let mut user_map = HashMap::new();
    user_map.insert(
        "aaa".to_string(),
        User {
            username: "Andy".to_string(),
        },
    );
    user_map.insert(
        "bbb".to_string(),
        User {
            username: "Bella".to_string(),
        },
    );
    user_map.insert(
        "ccc".to_string(),
        User {
            username: "Callie".to_string(),
        },
    );
    user_map.insert(
        "ddd".to_string(),
        User {
            username: "Daren".to_string(),
        },
    );
    user_map
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<Self>()
            .expect("User not found. Did you add auth_middleware?");
        Ok(user.clone())
    }
}

pub async fn auth_middleware(
    State(user_map): State<Arc<UserMap>>,
    mut request: Request,
    next: Next,
) -> axum::response::Result<Response> {
    let bearer = request
        .extract_parts::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let token = bearer.token();

    let user = user_map.get(token).ok_or(StatusCode::UNAUTHORIZED)?;
    request.extensions_mut().insert(user.clone());

    Ok(next.run(request).await)
}

pub fn main() {


println!("--------------------  ");
{

//mod auth;

use std::sync::Arc;

use axum::{middleware::from_fn_with_state, routing::get, Router};

//use auth::{auth_middleware, build_user_map, User, UserMap};


#[tokio::main]
async fn main() {
    let user_map = build_user_map();
    let app = build_app(Arc::new(user_map));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[rustfmt::skip]
fn build_app(user_map: Arc<UserMap>) -> Router {
    let public_router = Router::new()
        .route("/public", get(public));

    let private_router = Router::new()
        .route("/private", get(private))
        .route("/your-name", get(your_name))
        .route_layer(from_fn_with_state(user_map.clone(), auth_middleware));

    Router::new()
        .nest("/", public_router)
        .nest("/", private_router)
        .with_state(user_map)
}

async fn public() -> &'static str {
    "This is public.\n"
}

async fn private() -> &'static str {
    "This is private.\n"
}

async fn your_name(user: User) -> String {
    format!("Your name is {}.\n", user.username)
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
$ curl -v http://localhost:3000/response
-------------------------------------------------- auth
$ curl http://localhost:3000/public
This is public.

$ curl -I http://localhost:3000/private
HTTP/1.1 400 Bad Request
content-length: 0
date: Mon, 27 May 2024 09:51:24 GMT

$ curl -H 'Authorization: Bearer aaa' http://localhost:3000/private
This is private.

$ curl -I http://localhost:3000/your-name
HTTP/1.1 400 Bad Request
content-length: 0
date: Mon, 27 May 2024 09:53:13 GMT

$ curl -H 'Authorization: Bearer bbb' http://localhost:3000/your-name
Your name is Bella.

");
main();

}

}

