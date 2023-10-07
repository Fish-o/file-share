#[macro_use]
extern crate rocket;

use std::{
  collections::HashMap,
  fs::File,
  path::{Path, PathBuf},
};

use log::trace;
use rocket::{fs::NamedFile, get, launch, routes, State};
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
async fn retrieve(db: &State<Database>, id: String) -> Option<NamedFile> {
  println!("ID: {}", id);
  let value = db.id_to_path.get(&id);
  println!("Value: {:?}", value);
  if value.is_none() {
    println!("No file found for id: {}", id);
    return None;
  }
  let file_name: String = value.unwrap().to_string();
  println!("File name: {}", file_name);
  let path = Path::new(&data_dir()).join("file-store").join(file_name);
  println!("Path: {:?}", path);
  NamedFile::open(path).await.ok()
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


"s04e01.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e01.720p.bluray.x264-shortbrehd.mkv",
"s04e02.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e02.720p.bluray.x264-shortbrehd.mkv",
"s04e03.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e03.720p.bluray.x264-shortbrehd.mkv",
"s04e04.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e04.720p.bluray.x264-shortbrehd.mkv",
"s04e05.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e05.720p.bluray.x264-shortbrehd.mkv",
"s04e06.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e06.720p.bluray.x264-shortbrehd.mkv",
"s04e07.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e07.720p.bluray.x264-shortbrehd.mkv",
"s04e08.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e08.720p.bluray.x264-shortbrehd.mkv",
"s04e09.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e09.720p.bluray.x264-shortbrehd.mkv",
"s04e10.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e10.720p.bluray.x264-shortbrehd.mkv",
"s04e11.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e11.720p.bluray.x264-shortbrehd.mkv",
"s04e12.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e12.720p.bluray.x264-shortbrehd.mkv",
"s04e13.mkv":"completed/Doctor.Who.2005.S04.Season.4.720p.BluRay.x264-SHORTBREHD [PublicHD]/doctor.who.2005.s04e13.720p.bluray.x264-shortbrehd.mkv",

"s05e01.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E01.720p.BluRay.x264-BiA.mkv",
"s05e02.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E02.720p.BluRay.x264-BiA.mkv",
"s05e03.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E03.720p.BluRay.x264-BiA.mkv",
"s05e04.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E04.720p.BluRay.x264-BiA.mkv",
"s05e05.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E05.720p.BluRay.x264-BiA.mkv",
"s05e06.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E06.720p.BluRay.x264-BiA.mkv",
"s05e07.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E07.720p.BluRay.x264-REWARD.mkv",
"s05e08.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E08.720p.BluRay.x264-REWARD.mkv",
"s05e09.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E09.720p.BluRay.x264-REWARD.mkv",
"s05e10.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E10.720p.BluRay.x264-BiA.mkv",
"s05e11.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E11.720p.BluRay.x264-BiA.mkv",
"s05e12.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E12.720p.BluRay.x264-BiA.mkv",
"s05e13.mkv":"completed/Doctor.Who.2005.S05.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S05E13.720p.BluRay.x264-BiA.mkv",


"s06eSpecial.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.a.christmas.carol.2010.special.720p.bluray.x264-shortbrehd.mkv",
"s06e01.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e01.720p.bluray.x264-shortbrehd.mkv",
"s06e02.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e02.720p.bluray.x264-shortbrehd.mkv",
"s06e03.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e03.720p.bluray.x264-shortbrehd.mkv",
"s06e04.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e04.720p.bluray.x264-shortbrehd.mkv",
"s06e05.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e05.720p.bluray.x264-shortbrehd.mkv",
"s06e06.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e06.720p.bluray.x264-shortbrehd.mkv",
"s06e07.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e07.720p.bluray.x264-shortbrehd.mkv",
"s06e08.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e08.720p.bluray.x264-shortbrehd.mkv",
"s06e09.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e09.720p.bluray.x264-shortbrehd.mkv",
"s06e10.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e10.720p.bluray.x264-shortbrehd.mkv",
"s06e11.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e11.720p.bluray.x264-shortbrehd.mkv",
"s06e12.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e12.720p.bluray.x264-shortbrehd.mkv",
"s06e13.mkv":"completed/Doctor.Who.2005.S06.720p.BluRay.x264-SHORTBREHD[rartv]/doctor.who.2005.s06e13.720p.bluray.x264-shortbrehd.mkv",

"s07ePrequel.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/Specials/doctor.who.2005.christmas.special.the.doctor.the.widow.and.the.wardrobe.2011.720p.bluray.x264-shortbrehd.mkv",
"s07eSpecial.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/Specials/Doctor.Who.2005.2012.Christmas.Special.The.Snowmen.720p.BluRay.X264-TRiPS.mkv",

"s07e01.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e01.720p.bluray.x264-bia.mkv",
"s07e02.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e02.720p.bluray.x264-bia.mkv",
"s07e03.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e03.720p.bluray.x264-bia.mkv",
"s07e04.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e04.720p.bluray.x264-bia.mkv",
"s07e05.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e05.720p.bluray.x264-bia.mkv",
"s07e06.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/trips-doctor.who.s07e06.mkv",
"s07e07.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/trips-doctor.who.s07e07.mkv",
"s07e08.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/trips-doctor.who.s07e08.mkv",
"s07e09.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e09.720p.bluray.x264-bia.mkv",
"s07e10.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e10.720p.bluray.x264-bia.mkv",
"s07e11.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e11.720p.bluray.x264-bia.mkv",
"s07e12.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e12.720p.bluray.x264-bia.mkv",
"s07e13.mkv":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s07e13.720p.bluray.x264-bia.mkv",
"s07eTheDayOfTheDoctor":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/doctor.who.the.day.of.the.doctor.2013.720p.bluray.dts.x264-hds.mkv",
"s07eTheTimeOfTheDoctor":"completed/Doctor.Who.2005.S07.720p.BluRay.x264-MIXED[rartv]/Doctor.Who.The.Time.of.the.Doctor.2013.720p.BluRay.x264-SADPANDA.mkv",

"s08eSpecial.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08.Christmas.Special.1080p.BluRay.x264-RRH.mkv",
"s08e01.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s08e01.1080p.bluray.x264-shortbrehd.mkv",
"s08e02.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E02.1080p.BluRay.x264-RRH.mkv",
"s08e03.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E03.1080p.BluRay.x264-RRH.mkv",
"s08e04.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E04.1080p.BluRay.x264-RRH.mkv",
"s08e05.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E05.1080p.BluRay.x264-RRH.mkv",
"s08e06.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E06.1080p.BluRay.x264-RRH.mkv",
"s08e07.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E07.1080p.BluRay.x264-RRH.mkv",
"s08e08.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E08.1080p.BluRay.x264-RRH.mkv",
"s08e09.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E09.1080p.BluRay.x264-RRH.mkv",
"s08e10.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E10.1080p.BluRay.x264-RRH.mkv",
"s08e11.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E11.1080p.BluRay.x264-RRH.mkv",
"s08e12.mkv":"completed/Doctor.Who.2005.S08.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.S08E12.1080p.BluRay.x264-RRH.mkv",

"s09ePrequel.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.Christmas.Special.2014.Last.Christmas.1080p.WEB-DL.DD5.1.H.mkv",
"s09eSpecial.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/Doctor.Who.2005.Christmas.Special.2015.1080p.BluRay.x264-SHORTBREHD.mkv",
"s09e01.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e01.1080p.bluray.x264-snooze.mkv",
"s09e02.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e02.1080p.bluray.x264-snooze.mkv",
"s09e03.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e03.1080p.bluray.x264-snooze.mkv",
"s09e04.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e04.1080p.bluray.x264-shortbrehd.mkv",
"s09e05.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e05.1080p.bluray.x264-snooze.mkv",
"s09e06.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e06.1080p.bluray.x264-shortbrehd.mkv",
"s09e07.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e07.1080p.bluray.x264-cadaver.mkv",
"s09e08.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e08.1080p.bluray.x264-cadaver.mkv",
"s09e09.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e09.1080p.bluray.x264-cadaver.mkv",
"s09e10.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e10.1080p.bluray.x264-cadaver.mkv",
"s09e11.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e11.1080p.bluray.x264-cadaver.mkv",
"s09e12.mkv":"completed/Doctor.Who.2005.S09.1080p.BluRay.x264-MIXED[rartv]/doctor.who.2005.s09e12.1080p.bluray.x264-cadaver.mkv",

// https://tardis.fandom.com/wiki/The_Return_of_Doctor_Mysterio_(TV_story)

"s10e01.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E01.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e02.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E02.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e03.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E03.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e04.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E04.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e05.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E05.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e06.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E06.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e07.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E07.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e08.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E08.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e09.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E09.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e10.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E10.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e11.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E11.1080p.BluRay.x264-SHORTBREHD.mkv",
"s10e12.mkv":"completed/Doctor.Who.2005.S10.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S10E12.1080p.BluRay.x264-SHORTBREHD.mkv",
// https://tardis.fandom.com/wiki/Twice_Upon_a_Time_(TV_story)

"s11e01.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E01.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e02.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E02.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e03.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E03.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e04.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E04.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e05.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E05.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e06.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E06.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e07.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E07.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e08.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E08.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e09.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E09.1080p.BluRay.x264-SHORTBREHD.mkv",
"s11e10.mkv":"completed/Doctor.Who.2005.S11.1080p.BluRay.x264-SHORTBREHD[rartv]/Doctor.Who.2005.S11E10.1080p.BluRay.x264-SHORTBREHD.mkv",

"s12e01.mkv":"completed/Doctor.Who.2005.S12E01.1080p.BluRay.x264-SHORTBREHD.mkv"
Doctor.Who.2005.S12.1080p.BluRay.x264-SHORTBREHD[rartv]/
Doctor.Who.2005.S13.1080p.AMZN.WEBRip.DDP5.1.x264-MIXED[rartv]/
 */
