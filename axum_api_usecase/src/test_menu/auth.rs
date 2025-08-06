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

