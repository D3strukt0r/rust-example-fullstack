use dioxus::prelude::*;
mod controllers;
mod components;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(components::App);
}
