extern crate minimal;
extern crate plaster;
#[macro_use]
extern crate log;
extern crate console_log;
extern crate wasm_bindgen;

use minimal::Model;
use plaster::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init();
    info!("Starting...");
    plaster::run::<Model>();
}

fn main() {}
