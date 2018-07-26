pub mod model;
pub mod schema;
pub mod auth;

use rocket;
use rocket_contrib::{Json, Value};
use self::model::User;
use db;
use self::auth::ApiKey;
use self::auth::crypto::sha2::Sha256;
use self::auth::jwt::{
    Header,
    Registered,
    Token,
};

#[post("/", data = "<user>")]
fn create(user: Json<User>, connection: db::Connection) -> Json<User> {
    let insert = User { id: None, ..user.into_inner() };
    Json(User::create(insert, &connection))
}

#[get("/")]
fn read(key: ApiKey, connection: db::Connection) -> Json<Value> {
    Json(json!(User::read(0, &connection)))
}

#[get("/", rank = 2)]
fn read_error() -> Json<Value> {
    Json(json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    ))
}

#[get("/<id>")]
fn read_one(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!(User::read(id, &connection)))
}

#[put("/<id>", data = "<user>")]
fn update(id: i32, user: Json<User>, connection: db::Connection) -> Json<Value> {
    let update = User { id: Some(id), ..user.into_inner() };
    Json(json!({
        "success": User::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": User::delete(id, &connection)
    }))
}

/* #[get("/sensitive")]
fn sensitive(key: ApiKey) -> String {
    format!("Hello, you have been identified as {}", key.0)
} */

#[derive(Serialize, Deserialize)]
struct Credentials {
   username: String,
   password: String
}

#[post("/login", data = "<credentials>")]
fn login(credentials: Json<Credentials>, connection: db::Connection) ->  Json<Value> {
    let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();
    
    match User::by_username_and_password(username, password, &connection) {
        None => { Json(json!({"success": false})) },
        Some(user) => {
            let claims = Registered {
                sub: Some(user.name.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            let message = token.signed(b"secret_key", Sha256::new()).ok().unwrap();
            Json(json!({
                "success": true,
                "token": message
            }))
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![read, read_error, read_one, create, update, delete])
        .mount("/auth", routes![login])
}