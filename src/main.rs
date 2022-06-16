mod components;
mod services;

use crate::components::main::Main;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
