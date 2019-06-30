#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::config::{Config, Environment};

use std::thread;

mod socket;
use crate::socket::Server;
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket_contrib::serve::StaticFiles;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

/*
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
*/
fn main() {
    println!("Hello, world!");
    let mut ws = ws::WebSocket::new(|out| Server { out }).unwrap();

    thread::spawn(|| { ws.listen("127.0.0.1:8080").unwrap(); });

   // rocket::ignite().mount("/", routes![index]).launch();
    let config = Config::build(Environment::Production)
        .address("127.0.0.1")
        .port(8081)
        .finalize().unwrap();

    rocket::custom(config)     // replaces calls to rocket::ignite() & Rocket.toml
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![index])
        .attach(Template::fairing())
   //     .attach(AdHoc::on_attach("Assets Config", |rocket| {
   //         let assets_dir = rocket.config()
   //             .get_str("assets_dir")
   //             .unwrap_or("assets/")
   //             .to_string();

   //         Ok(rocket.manage(AssetsDir(assets_dir)))
   //     }))
        .launch();

}

