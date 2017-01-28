#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate asciii;

use std::path::{Path,PathBuf};

use rocket::response::NamedFile;
use rocket::response::content;
use rocket::response::content::{Plain,Content};
use rocket::http::ContentType;

use asciii::actions;
use asciii::storage::StorageDir;

#[get("/<file..>")]
fn doc(file: PathBuf) -> Option<NamedFile> {
    let p = Path::new("./target/doc/").join(file);
    let target = if p.is_dir() {
        p.join("index.html")
    } else { p };

    println!("opening {:?}", target);
    NamedFile::open(target).ok()
}

#[derive(FromForm, Debug)]
struct Dir {
    year: Option<i32>,
    tasks: Option<bool>,
    all: Option<bool>,
}

#[get("/", rank=2)]
fn cal() -> Result<Content<String>, String> {
    cal_params(Dir{year:None,tasks:None,all:None})
}

#[get("/", rank=2)]
fn cal_plain() -> Result<Plain<String>, String> {
    cal_plain_params(Dir{year:None,tasks:None,all:None})
}

#[get("/?<dir>", rank=1)]
fn cal_params(dir:Dir) -> Result<Content<String>, String> {
    let storage_dir = match dir {
        Dir{tasks: _, all: Some(true), year:None} => StorageDir::All,
        Dir{tasks: _, all: Some(true), year:Some(_)} => return Err("Ambiguous".into()),
        Dir{tasks: _, all: None, year:Some(year)} => StorageDir::Archive(year),
        Dir{tasks: _, all: None, year:None} => StorageDir::Working,
        _ => StorageDir::Working,
    };

    let tasks = dir.tasks == Some(true);

    actions::calendar(storage_dir, tasks)
        .map(|s| Content(ContentType::new("text", "calendar"),s) )
        .map_err(|_|String::from("error"))
}

#[get("/?<dir>", rank=1)]
fn cal_plain_params(dir:Dir) -> Result<Plain<String>, String> {
    let storage_dir = match dir {
        Dir{tasks: _, all: Some(true), year:None} => StorageDir::All,
        Dir{tasks: _, all: Some(true), year:Some(_)} => return Err("Ambiguous".into()),
        Dir{tasks: _, all: None, year:Some(year)} => StorageDir::Archive(year),
        Dir{tasks: _, all: None, year:None} => StorageDir::Working,
        _ => StorageDir::Working,
    };

    let tasks = dir.tasks == Some(true);

    actions::calendar(storage_dir, tasks)
        .map(|s| content::Plain(s) )
        .map_err(|_|String::from("error"))
}

fn main() {
    rocket::ignite()
        .mount("/cal/plain", routes![cal_plain,cal_plain_params])
        .mount("/cal", routes![cal,cal_params])
        .mount("/doc", routes![doc])
        .launch();
}
