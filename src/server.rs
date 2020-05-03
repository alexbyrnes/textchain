use crate::model::Block;
use crate::node::Node;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct Response {
    status: String,
    data: Block,
}

#[derive(Deserialize)]
pub struct Request {
    text: String,
}

pub struct Server {}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn start(&self) {
        let node = Node::new();
        node.start();

        rocket::ignite()
            .manage(node)
            .mount("/", routes![add, files])
            .launch();
    }
}

#[post("/add", format = "json", data = "<request>")]
fn add(node: State<Node>, request: Json<Request>) -> Json<Response> {
    let status = "success".into();
    let block = node.add(request.text.clone());

    Json(Response {
        status,
        data: block,
    })
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("ui/").join(file)).ok()
}
