#[macro_use]
extern crate rocket;

use std::{
  collections::HashMap,
  fs::File,
  path::{Path, PathBuf},
};

use log::trace;
use rocket::{get, launch, routes, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Database {
  id_to_path: HashMap<String, String>,
}
fn data_dir() -> PathBuf {
  let data_dir = std::env::var("DATA_DIR")
    .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data").to_string());
  Path::new(&data_dir).to_path_buf()
}

#[launch]
async fn rocket() -> _ {
  pretty_env_logger::init();
  trace!("Initialized logger");

  let config_file_path = Path::new(&data_dir()).join("config.json");

  let db = if config_file_path.exists() {
    let file = File::open(config_file_path).unwrap();
    let db: Database = serde_json::from_reader(file).unwrap();
    db
  } else {
    let db = Database {
      id_to_path: HashMap::new(),
    };
    let file = File::create(config_file_path).unwrap();
    serde_json::to_writer(file, &db).unwrap();
    db
  };

  rocket::build()
    .manage(db)
    .mount("/", routes![home, retrieve])
}

#[get("/")]
fn home() -> String {
  "Hello, world!".to_string()
}

#[get("/dr-who/<id>")]
async fn retrieve(db: &State<Database>, id: String) -> Option<File> {
  let value = db.id_to_path.get(&id);
  if value.is_none() {
    return None;
  }
  let file_name: String = value.unwrap().to_string();
  let path = Path::new(&data_dir()).join("file-store").join(file_name);
  File::open(path).ok()
}
