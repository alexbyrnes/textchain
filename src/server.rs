use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct Response {
    result: String
}

#[get("/add")]
pub fn add() -> Json<Response> {
    let result = "ok".into();
    Json(Response { result })
}


#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("ui/").join(file)).ok()
}

