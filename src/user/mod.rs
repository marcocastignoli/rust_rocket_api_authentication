pub mod model;
pub mod schema;
pub mod auth;

use rocket::{self, http::Status};
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
fn create(user: Json<User>, connection: db::Connection) -> Result<Json<User>, Status> {
    let insert = User { id: None, ..user.into_inner() };
    User::create(insert, &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/info")]
fn info(key: ApiKey) -> Json<Value> {
    Json(json!(
        {
            "success": true,
            "message": key.0
        }
    ))
}

#[get("/info", rank = 2)]
fn info_error() -> Json<Value> {
    Json(json!(
        {
            "success": false,
            "message": "Not authorized"
        }
    ))
}

#[get("/")]
fn read(_key: ApiKey, connection: db::Connection) -> Result<Json<Value>, Status> {
    User::read(0, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
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
fn read_one(id: i32, connection: db::Connection) -> Result<Json<Value>, Status> {
    User::read(id, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
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
fn login(credentials: Json<Credentials>, connection: db::Connection) ->  Result<Json<Value>, Status> {
    let header: Header = Default::default();
    let username = credentials.username.to_string();
    let password = credentials.password.to_string();
    
    match User::by_username_and_password(username, password, &connection) {
        None => {
            Err(Status::NotFound)
        },
        Some(user) => {
            let claims = Registered {
                sub: Some(user.name.into()),
                ..Default::default()
            };
            let token = Token::new(header, claims);

            token.signed(b"secret_key", Sha256::new())
                .map(|message| Json(json!({ "success": true, "token": message })))
                .map_err(|_| Status::InternalServerError)
        }
    }
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket
        .mount("/user", routes![read, read_error, read_one, create, update, delete, info, info_error])
        .mount("/auth", routes![login])
}