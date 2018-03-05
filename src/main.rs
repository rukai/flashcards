#![feature(plugin, decl_macro, custom_derive)]
#![plugin(rocket_codegen)]

             extern crate rocket_contrib;
             extern crate rocket;
#[macro_use] extern crate serde_derive;
             extern crate serde_json;

use rocket_contrib::Template;
use rocket::response::NamedFile;

use std::path::{Path, PathBuf};
use std::env;
use std::ffi::OsStr;

#[derive(Serialize, Clone)]
pub struct Card {
    front: String,
    back: String,
}

#[derive(Serialize, Clone)]
pub struct Page {
    cards_json: String,
}

#[get("/")]
fn index() -> Template {
    filter(String::from(""))
}

#[get("/<request>")]
fn filter(request: String) -> Template {
    let mut cards = vec!();
    cards.push(Card {
        front: String::from("FOO"),
        back: String::from("BAR")
    });

    cards.push(Card {
        front: String::from("ひらがな"),
        back: String::from("hiragana")
    });

    let cards_json = serde_json::to_string(&cards).unwrap();
    let page = Page { cards_json };

    Template::render("flashcards", page)
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    if env::current_dir().unwrap().file_name().unwrap() != OsStr::new("flashcards") {
        // templates are correctly located by finding the Rocket.toml
        // However static files are not handled, so if we aren't in the root directory, close immediately to avoid headaches.
        println!("Wrong directory, dummy!");
        return;
    }
    rocket::ignite()
        .mount("/", routes![index, filter, files])
        .attach(Template::fairing())
        .launch();
}

pub fn get_path() -> PathBuf {
    match env::home_dir() {
        Some (mut home) => {
            home.push("Flashcards");
            home
        }
        None => {
            panic!("could not get path of home");
        }
    }
}
