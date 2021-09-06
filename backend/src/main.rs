#![feature(proc_macro_hygiene, decl_macro, int_roundings)]

#[macro_use] extern crate rocket;

use std::{fs, io};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use deltalake;

use rocket::serde::json::Json;

fn is_delta(path: &PathBuf) -> bool {
    let path_check = path.join("_delta_log");
    if path_check.exists() {
        true
    } else {
        false
    }
}

fn find_delta_tables<P>(root: P, tables: &mut HashMap<String, String>) where P:AsRef<Path>{
    for e in fs::read_dir(root).unwrap() {
        let path = e.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();
        if metadata.is_dir() {
            //check if delta table
            if is_delta(&path) {
                //insert to hashmap
                tables.insert(
                path.clone().file_name().unwrap().to_str().unwrap().to_string(), 
                path.clone().to_str().unwrap().to_string()
                );
            }
            else {
                find_delta_tables(&path, tables);
            }
        }
    }
}

#[get("/tables")]
async fn get_tables() -> Json<HashMap<String,String>> {
    let mut tables: HashMap<String, String> = HashMap::new();

    let root = "./";
    find_delta_tables(root, &mut tables);
    
    Json(tables)
}

#[get("/table/files")]
async fn files() -> Json<Vec<String>> {
    let uri = "./tests/data/simple_table";
    let table = deltalake::open_table(uri).await.unwrap();
    let files = table.get_files().iter().map(|s| s.to_string()).collect();
    Json(files)
}

#[get("/")]
async fn home() -> &'static str {
    "Hello, world"
}

#[launch]
async fn rocket() -> _ {
    // add state for tables hashmap to be used later
    // create set early
    // endpoint can just return the state
    rocket::build().mount("/", routes![home, files, get_tables])
}