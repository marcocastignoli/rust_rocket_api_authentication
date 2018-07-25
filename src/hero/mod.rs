pub mod model;
pub mod schema;

use rocket;
use rocket_contrib::{Json, Value};
use self::model::Hero;


use db;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero { id: None, ..hero.into_inner() };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(0, &connection)))
}

#[get("/<id>")]
fn readOne(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(id, &connection)))
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
    rocket.mount("/hero", routes![read, readOne, create, update, delete])
}