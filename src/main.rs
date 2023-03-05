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
/*
"s01e01.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e01.1080p.bluray.x264-shortbrehd.mkv",
"s01e02.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e02.1080p.bluray.x264-shortbrehd.mkv",
"s01e03.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e03.1080p.bluray.x264-shortbrehd.mkv",
"s01e04.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e04.1080p.bluray.x264-shortbrehd.mkv",
"s01e05.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e05.1080p.bluray.x264-shortbrehd.mkv",
"s01e06.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e06.1080p.bluray.x264-shortbrehd.mkv",
"s01e07.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e07.1080p.bluray.x264-shortbrehd.mkv",
"s01e08.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e08.1080p.bluray.x264-shortbrehd.mkv",
"s01e09.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e09.1080p.bluray.x264-shortbrehd.mkv",
"s01e10.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e10.1080p.bluray.x264-shortbrehd.mkv",
"s01e11.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e11.1080p.bluray.x264-shortbrehd.mkv",
"s01e12.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e12.1080p.bluray.x264-shortbrehd.mkv",
"s01e13.mkv": "completed/Doctor.Who.2005.S01.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s01e13.1080p.bluray.x264-shortbrehd.mkv",



"s02eSpecial.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.christmas.special.2005.1080p.bluray.x264-shortbrehd.mkv",
"s02e01.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e01.1080p.bluray.x264-shortbrehd.mkv",
"s02e02.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e02.1080p.bluray.x264-shortbrehd.mkv",
"s02e03.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e03.1080p.bluray.x264-shortbrehd.mkv",
"s02e04.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e04.1080p.bluray.x264-shortbrehd.mkv",
"s02e05.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e05.1080p.bluray.x264-shortbrehd.mkv",
"s02e06.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e06.1080p.bluray.x264-shortbrehd.mkv",
"s02e07.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e07.1080p.bluray.x264-shortbrehd.mkv",
"s02e08.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e08.1080p.bluray.x264-shortbrehd.mkv",
"s02e09.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e09.1080p.bluray.x264-shortbrehd.mkv",
"s02e10.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e10.1080p.bluray.x264-shortbrehd.mkv",
"s02e11.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e11.1080p.bluray.x264-shortbrehd.mkv",
"s02e12.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e12.1080p.bluray.x264-shortbrehd.mkv",
"s02e13.mkv":"completed/Doctor.Who.2005.S02.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s02e13.1080p.bluray.x264-shortbrehd.mkv",

"s03eSpecial.mkv": "completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.christmas.special.2006.1080p.bluray.x264-shortbrehd.mkv",
"s03e01.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e01.1080p.bluray.x264-shortbrehd.mkv",
"s03e02.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e02.1080p.bluray.x264-shortbrehd.mkv",
"s03e03.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e03.1080p.bluray.x264-shortbrehd.mkv",
"s03e04.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e04.1080p.bluray.x264-shortbrehd.mkv",
"s03e05.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e05.1080p.bluray.x264-shortbrehd.mkv",
"s03e06.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e06.1080p.bluray.x264-shortbrehd.mkv",
"s03e07.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e07.1080p.bluray.x264-shortbrehd.mkv",
"s03e08.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e08.1080p.bluray.x264-shortbrehd.mkv",
"s03e09.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e09.1080p.bluray.x264-shortbrehd.mkv",
"s03e10.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e10.1080p.bluray.x264-shortbrehd.mkv",
"s03e11.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e11.1080p.bluray.x264-shortbrehd.mkv",
"s03e12.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e12.1080p.bluray.x264-shortbrehd.mkv",
"s03e13.mkv":"completed/Doctor.Who.2005.S03.1080p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s03e13.1080p.bluray.x264-shortbrehd.mkv",


Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/
Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/
Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/
Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/
Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/
Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/
Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/
Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/
Doctor.Who.2005.S12.1080p.BluRay.x264-SHORTBREHD[rartv]/
Doctor.Who.2005.S13.1080p.AMZN.WEBRip.DDP5.1.x264-MIXED[rartv]/
 */
