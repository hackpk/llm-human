#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;


fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        "hello "
    }
}

