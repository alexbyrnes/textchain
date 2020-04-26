use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct Response {
    status: String,
    data: String,
}

#[derive(Deserialize)]
pub struct Request {
    text: String,
}

#[post("/add", format = "json", data = "<request>")]
pub fn add(request: Json<Request>) -> Json<Response> {
    let status = "success".into();
    Json(Response {
        status,
        data: request.text.clone(),
    })
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("ui/").join(file)).ok()
}
