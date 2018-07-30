pub mod model;
pub mod schema;

use rocket::{self, http::Status};
use rocket_contrib::{Json, Value};
use self::model::Hero;


use db;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Result<Json<Hero>, Status> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Hero::create(insert, &connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
fn read(connection: db::Connection) -> Result<Json<Value>, Status> {
    Hero::read(0, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[get("/<id>")]
fn read_one(id: i32, connection: db::Connection) -> Result<Json<Value>, Status> {
    Hero::read(id, &connection)
        .map(|item| Json(json!(item)))
        .map_err(|_| Status::NotFound)
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<Value> {
    let update = Hero { id: Some(id), ..hero.into_inner() };
    Json(json!({
        "success": Hero::update(id, update, &connection)
    }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({
        "success": Hero::delete(id, &connection)
    }))
}

pub fn mount(rocket: rocket::Rocket) -> rocket::Rocket {
    rocket.mount("/hero", routes![read, read_one, create, update, delete])
}