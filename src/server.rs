use rocket::response::NamedFile;
use std::path::{Path, PathBuf};

#[get("/add")]
pub fn add() -> &'static str {
    "ok"
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("ui/").join(file)).ok()
}
