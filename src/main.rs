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
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Clone)]
pub struct Card {
    front: String,
    back: String,
}

#[derive(Serialize, Clone)]
pub struct Page {
    request: String,
    cards_json: String,
}

#[get("/")]
fn index() -> Template {
    filter(String::from(""))
}

#[get("/<request>")]
fn filter(request: String) -> Template {
    let mut file = match File::open("flashcards.txt") {
        Ok(file) => file,
        Err(err) => panic!("Failed to open flashcards.txt: {}", err)
    };
    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        panic!("Failed to read flashcards.txt: {}", err)
    };

    let mut section = String::new();
    let mut cards = vec!();
    let mut flip = false;
    for line in contents.lines() {
        let line = line.trim();
        if line.len() == 0 { }
        else if !line.contains("=") {
            flip = line.starts_with('?');
            section = String::from(line.trim_left_matches('?'));
        }
        else if section.starts_with(&request) {
            let mut sides = line.split("=");
            let front = String::from(sides.next().unwrap());
            let back = String::from(sides.next().unwrap());

            if flip {
                cards.push(Card {
                    front: back.clone(),
                    back: front.clone(),
                });
            }

            cards.push(Card {
                front,
                back
            });
        }
    }

    let cards_json = serde_json::to_string(&cards).unwrap();
    let page = Page { cards_json, request };

    Template::render("flashcards", page)
}

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    if env::current_dir().unwrap().file_name().unwrap() != OsStr::new("data") {
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
