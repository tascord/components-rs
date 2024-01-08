use dominator::{html, Dom};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::console_warn;

pub fn none_dom() -> Dom {
    console_warn!("Option value was None, returning placeholder DOM element");
    html!("div", {
        .class("hidden")
        .text("[[ None value found. Placeholder generated ]]")
    })
}

pub fn rand_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

pub fn parent_or_self(e: HtmlElement) -> HtmlElement {
    e.parent_element()
        .unwrap_or(e.clone().into())
        .dyn_into::<HtmlElement>()
        .unwrap_or(e)
}
