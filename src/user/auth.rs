#![feature(globs)]
use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};

pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;

use std::default::Default;
use user::auth::crypto::sha2::Sha256;
use self::jwt::{
    Header,
    Registered,
    Token,
};


pub struct ApiKey(pub String);

pub fn is_valid(key: &str) -> String {
    let token = Token::<Header, Registered>::parse(key).unwrap();
    if(token.verify(b"secret_key", Sha256::new())){
        token.claims.sub.unwrap()
    } else {
        "0".to_string()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }

        let key = keys[0];

        let checker = "0";
        let value = is_valid(keys[0]);
        
        if value == "0" {
            return Outcome::Forward(());
        }

        return Outcome::Success(ApiKey(value));
    }
}