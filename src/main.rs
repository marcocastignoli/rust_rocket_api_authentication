// Dico a Rust di utilizzare il suo sistema di plugin
#![feature(plugin)]
// Dico a Rust di utilizzare il plugin "rocket_codegen"
#![plugin(rocket_codegen)]
// Utilizzare extern crate per include moduli dall'esterno
extern crate rocket;
// Specificare #[macro_use] per importare anche le macro
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
// Utilizzare mod per includere moduli nel filesystem dell'applicazione
mod db;

// Includo i moduli
mod user;
mod hero;
fn main() {
    let mut rocket = rocket::ignite()
        .manage(db::connect());
    rocket = user::mount(rocket);
    rocket = hero::mount(rocket);
    rocket.launch();
}