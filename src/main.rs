#![feature(proc_macro_hygiene, decl_macro)]

use clap::{App, Arg};
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::collections::HashMap;
use std::fs;
use std::iter::Iterator;
use std::option::Option;

#[macro_use]
extern crate rocket;

type Id = String;

type LanguageId = String;

type Definition = String;

#[derive(Serialize, Deserialize, Clone)]
struct Word {
    uuid: Id,
    definitions: HashMap<LanguageId, Vec<Definition>>,
    synonyms: HashMap<LanguageId, Vec<Id>>,
    antonyms: HashMap<LanguageId, Vec<Id>>,
    dependencies: Vec<Id>,
    dependers: Vec<Id>,
    description: String,
}

impl Word {
    fn new() -> Word {
        return Word {
            uuid: "".to_string(),
            definitions: HashMap::new(),
            synonyms: HashMap::new(),
            antonyms: HashMap::new(),
            dependencies: vec![],
            dependers: vec![],
            description: "".to_string(),
        };
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Dictionary {
    words: Vec<Word>,
}

impl Dictionary {
    fn new() -> Dictionary {
        return Dictionary { words: vec![] };
    }

    fn from_file(path: &str) -> Dictionary {
        let file_data = fs::read_to_string(path).expect("could not read file");
        let dict: Dictionary = serde_json::from_str(&file_data).expect("could not parse JSON");
        return dict;
    }

    fn find<P>(self, mut predicate: P) -> Option<Word>
    where
        P: FnMut(&Word) -> bool,
    {
        return self.words.into_iter().find(move |x| predicate(x));
    }

    fn find_all<P>(self, mut predicate: P) -> impl Iterator<Item = Word>
    where
        P: FnMut(&Word) -> bool,
    {
        return self.words.into_iter().filter(move |x| predicate(x));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search/<uuid>", format = "json")]
fn search_uuid(dict: State<Dictionary>, uuid: String) -> Json<Word> {
    return match dict.clone().find(|word| word.uuid == uuid.to_string()) {
        Some(word) => Json(word),
        // TODO use results with errors
        _ => Json(Word::new()),
    };
}

#[get("/search/<lang>/<word>", format = "json")]
fn search_word(dict: State<Dictionary>, lang: String, word: String) -> Json<Vec<Word>> {
    return Json(
        dict.clone()
            .find_all(|word_data| word_data.definitions[&lang].contains(&word))
            .collect(),
    );
}

fn main() {
    let matches = App::new("iskwet")
        .version("0.1.0")
        .arg(
            Arg::with_name("data")
                .required(true)
                .index(1)
                .about("a path to a .json file containing dictionary + thesaurus data"),
        )
        .get_matches();

    rocket::ignite()
        .manage(Dictionary::from_file(matches.value_of("data").unwrap()))
        .mount("/", routes![index, search_uuid, search_word])
        .launch();
}
