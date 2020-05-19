#![feature(proc_macro_hygiene, decl_macro)]

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rocket_contrib::uuid::Uuid;

#[macro_use]
extern crate rocket;

#[derive(Serialize, Deserialize)]
struct Word {
    uuid: String,
    definitions: HashMap<String, String>,
    dependencies: Vec<String>,
    dependers: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct Dictionary {
    words: Vec<Word>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<uuid>", format = "json")]
fn search_uuid(uuid: Uuid) -> String {
    return "searching for '".to_string() + &uuid.to_string() + "'";
}

#[get("/search/<lang>/<word>", format = "json")]
fn search_word(lang: String, word: String) -> String {
    return "searching for '".to_string() + &word + "' in language: " + &lang;
}

fn main() {
    rocket::ignite().mount("/", routes![index, search_uuid, search_word]).launch();
}
