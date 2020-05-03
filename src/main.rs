#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

mod mining;
mod model;
mod node;
mod serialization;
mod server;
mod util;
mod validation;

use server::Server;

fn main() {
    let server = Server::new();
    server.start();
}
