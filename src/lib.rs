use dominator::{html, Dom};
use std::rc::Rc;
use wasm_bindgen::prelude::wasm_bindgen;

extern crate custom_derive;
extern crate derive_builder;

pub mod components;

pub struct App {}
impl App {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .child(
                components::Button::default()
                .text("Hello World")
                .colour("orange")
                .build()
                .render()
            )
        })
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let model = Rc::new(App {});
    dominator::append_dom(&dominator::body(), model.render());
}
