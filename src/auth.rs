use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use dotenvy::dotenv;
use jsonwebtoken::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    exp: usize,
    resource_access: HashMap<String, HashMap<String, Vec<String>>>,
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub roles: Vec<Role>,
}

#[derive(Debug, PartialEq)]
pub enum Role {
    Guest,
    Reader,
    Writer,
    Admin,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(authn) = parts.headers.get(AUTHORIZATION) {
            dotenv().ok();
            let key = env::var("JWT_SECRET").expect("Failed to get public key");
            let jwt_header = authn.to_str().unwrap().split_whitespace().nth(1).unwrap();
            let token = decode::<Claims>(
                &jwt_header,
                &DecodingKey::from_rsa_pem(key.as_bytes()).unwrap(),
                &Validation::new(Algorithm::RS256),
            );
            if let Ok(t) = token {
                let role = t.claims.resource_access["api-test-client"]["roles"][0].clone();
                match role.clone().as_str() {
                    "writer" => Ok(AuthenticatedUser {
                        roles: vec![Role::Writer],
                    }),
                    "admin" => Ok(AuthenticatedUser {
                        roles: vec![Role::Admin],
                    }),
                    "reader" => Ok(AuthenticatedUser {
                        roles: vec![Role::Admin],
                    }),
                    _ => Err((
                        StatusCode::UNAUTHORIZED,
                        "User is unauthorized to make this request",
                    )),
                }
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid Bearer Token"))
            }
        } else {
            Err((StatusCode::BAD_REQUEST, "`AUTHORIZATION` header is missing"))
        }
    }
}
