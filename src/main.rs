#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod mining;
mod model;
mod serialization;
mod util;
mod validation;
mod server;

use mining::mine;
use model::{Block, BlockData};
use serialization::serialize_seed;
use util::epoch;
use validation::validate;
use server::*;

fn main() {
    // Start the http server
    rocket::ignite()
        .mount("/", routes![add, files])
        .launch();

    let mut chain: Vec<Block> = vec![];

    let genesis_date = epoch();

    let blockdata0 = BlockData::new(0, "genesis".to_string(), 0.to_string(), 0, genesis_date);
    let genesis = Block::new(blockdata0);

    chain.push(genesis);

    for _ in 1..8 {
        let last = chain.last().unwrap();

        let new_index = last.data.index + 1;
        let text = format!("{}{}", "the text", new_index);
        let prevhash = &last.hash;
        let datetime = epoch();

        // The seed is a serialized BlockData (essentially a header) without the nonce.
        let seed = serialize_seed(new_index, &text, &prevhash, datetime);
        let nonce = mine(seed);

        let blockdata = BlockData::new(new_index, text, prevhash.to_string(), nonce, datetime);

        let block = Block::new(blockdata);
        let valid = validate(&block, &last.hash);

        if valid {
            chain.push(block);
        }
    }

    chain.iter().for_each(|x| println!("{:#?}", x));
}
