use rocket::Outcome;
use rocket::request::{self, Request, FromRequest};

pub extern crate crypto;
pub extern crate jwt;
pub extern crate rustc_serialize;

use user::auth::crypto::sha2::Sha256;
use self::jwt::{
    Header,
    Registered,
    Token,
};


pub struct ApiKey(pub String);

pub fn read_token(key: &str) -> Result<String, String> {
    let token = Token::<Header, Registered>::parse(key)
        .map_err(|_| "Unable to parse key".to_string())?;
    if token.verify(b"secret_key", Sha256::new()) {
        token.claims.sub.ok_or("Claims not valid".to_string())
    } else {
        Err("Token not valid".to_string())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<ApiKey, ()> {
        let keys: Vec<_> = request.headers().get("Authentication").collect();
        if keys.len() != 1 {
            return Outcome::Forward(());
        }
        match read_token(keys[0]) {
            Ok(claim) => Outcome::Success(ApiKey(claim)),
            Err(_) => Outcome::Forward(())
        }
    }
}