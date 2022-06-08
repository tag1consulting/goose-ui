use yew::prelude::*;

mod components;
mod services;
use components::controls::Controls;

#[function_component(Main)]
fn main() -> Html {
    html! {
        <Controls />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
}
