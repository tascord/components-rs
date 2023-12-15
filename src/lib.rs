use crate::{
    components::{button::ButtonVariant, text::TextVariant, *},
    helpers::css::style_element,
};
use dominator::{html, Dom};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

pub mod components;
pub mod helpers;

pub fn display_case(element: Dom, label: &str) -> Dom {
    html!("div", {
        .style("margin", "1rem")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("align-items", "center")
        .style("justify-content", "center")
        .style("border", "1px solid #eeeeee")
        .child(html!("div", {
            .style("margin", "1rem")
            .child(element)
        }))
        .child(
            html!("h2", {
                .text(label)
                .style("border-top", "1px solid #eeeeee")
                .style("width", "100%")
                .style("text-align", "center")
                .style("padding", "0.5rem 0")
                .style("margin", "0")
                .style("font-size", "15px")
                .style("font-weight", "400")
            })
        )
    })
}

pub fn row(label: &str, elements: Vec<Dom>) -> Dom {
    html!("div", {
        .style("margin-top", "2rem")
        .style("width", "100%")
        .child(
            html!("h2", {
                .text(label)
                .style("padding-left", "1rem")
                .style("margin", "0")
                .style("font-size", "30px")
                .style("font-weight", "400")
            })
        )
        .child(
            html!("div", {
                .style("display", "flex")
                .style("flex-direction", "row")
                .style("align-items", "center")
                .style("justify-content", "center")
                .style("max-width", "calc(100vw - 2rem)")
                .style("overflow-x", "auto")
                .children(elements)
                })
        )
    })
}

pub struct App {}
impl App {
    pub fn render(self: Rc<Self>) -> Dom {
        html!("main", {
            .child(style_element())
            .child(row("Button", vec![
                display_case(Button::new().text("Hello, world!").ok(), "Button (Solid)"),
                display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Light).ok(), "Button (Light)"),
                display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Outline).ok(), "Button (Outline)"),
                display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Subtle).ok(), "Button (Subtle)"),
            ]))
            .child(row("Text", vec![
                display_case(Text::new().text("Hello, world!").variant(TextVariant::Subscript).ok(), "Text (Subcript)"),
                display_case(Text::new().text("Hello, world!").variant(TextVariant::Superscript).ok(), "Text (Superscript)"),
                display_case(Text::new().text("Hello, world!").variant(TextVariant::Small).ok(), "Text (Small)"),
                display_case(Text::new().text("Hello, world!").ok(), "Text (Default)"),
                display_case(Text::new().text("Hello, world!").variant(TextVariant::H4).ok(), "Text (H4)"),
                display_case(Text::new().text("Hello, world!").variant(TextVariant::H3).ok(), "Text (H3)"),
                display_case(Text::new().text("Hello, world!").variant(TextVariant::H2).ok(), "Text (H2)"),
                display_case(Text::new().text("Hello, world!").variant(TextVariant::H1).ok(), "Text (H1)"),
            ]))
            .child(row("Flex", vec![
                display_case(
                    Flex::new()
                        .child(Button::new().text("Hello,").ok())
                        .child(Button::new().text("world!").ok())
                        .ok(),
                    "Flex (Default)")
            ]))
        })
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let model = Rc::new(App {});
    dominator::append_dom(&dominator::body(), model.render());
}
