
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
use axum::{
    async_trait,
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, RequestPartsExt, Router,
};
use serde::Serialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let hc_router = Router::new().route("/", get(health_check));
    let mountain_router = Router::new().route("/", get(find_sacred_mountains));

    let app = Router::new()
        .nest("/:version/hc", hc_router)
        .nest("/:version/mountains", mountain_router);

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

async fn health_check(version: ApiVersion) -> StatusCode {
    println!("received request with version {:?}", version);

    StatusCode::OK
}

async fn find_sacred_mountains(version: ApiVersion) -> (StatusCode, Json<JsonResponse>) {
    println!("received request with version {:?}", version);

    let response: JsonResponse = Default::default();
    (StatusCode::OK, Json(response))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonResponse {
    mountains: Vec<EightThousander>,
    total: usize,
}

impl Default for JsonResponse {
    fn default() -> Self {
        let mountains = vec![
            EightThousander::new(1, "エベレスト".to_string(), 8848),
            EightThousander::new(2, "K2".to_string(), 8611),
            EightThousander::new(3, "カンチェンジェンガ".to_string(), 8586),
            EightThousander::new(4, "ローツェ".to_string(), 8516),
            EightThousander::new(5, "マカルー".to_string(), 8463),
            EightThousander::new(6, "チョ・オユー".to_string(), 8188),
            EightThousander::new(7, "ダウラギリ".to_string(), 8167),
            EightThousander::new(8, "マナスル".to_string(), 8163),
            EightThousander::new(9, "ナンガ・パルバット".to_string(), 8126),
            EightThousander::new(10, "アンナプルナ".to_string(), 8091),
            EightThousander::new(11, "ガッシャーブルⅠ峰".to_string(), 8080),
            EightThousander::new(12, "ブロード・ピーク".to_string(), 8051),
            EightThousander::new(13, "ガッシャーブルムⅡ峰".to_string(), 8035),
            EightThousander::new(14, "シシャパンマ".to_string(), 8027),
        ];
        let total = mountains.len();

        Self { mountains, total }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EightThousander {
    id: i32,
    name: String,
    elevation: i32,
}

impl EightThousander {
    fn new(id: i32, name: String, elevation: i32) -> Self {
        Self {
            id,
            name,
            elevation,
        }
    }
}

#[derive(Debug)]
enum ApiVersion {
    V1,
    V2,
    V3,
}

#[async_trait]
impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let version = params
            .get("version")
            .ok_or_else(|| (StatusCode::NOT_FOUND, "version param missing").into_response())?;

        match version.as_str() {
            "v1" => Ok(ApiVersion::V1),
            "v2" => Ok(ApiVersion::V2),
            "v3" => Ok(ApiVersion::V3),
            _ => Err((StatusCode::NOT_FOUND, "unknown version").into_response()),
        }
    }
}

println!("
$ curl http://localhost:3000/v1/hc
$ curl http://localhost:3000/v1/mountains | jq

$ curl http://localhost:3000/v2/hc
$ curl http://localhost:3000/v2/mountains | jq

$ curl http://localhost:3000/v3/hc
$ curl http://localhost:3000/v3/mountains | jq

$ curl http://localhost:3000/v4/hc
unknown version
$ curl http://localhost:3000/v4/mountains
unknown version

");

main();

}

}

