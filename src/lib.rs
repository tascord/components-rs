use crate::components::{
    button::ButtonVariant,
    text::TextVariant,
    ty::{Colour, RemSizing},
    *,
};
use components::shell::SidebarItem;
use dominator::{html, Dom};
use helpers::Provider;
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
        .style("width", "20vw")
        .style("min-width", "20rem")
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
                .attr("id", &label.to_lowercase())
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
                .style("flex-wrap", "wrap")
                .style("align-items", "center")
                .style("justify-content", "center")
                .children(elements)
                })
        )
    })
}

pub fn display() -> Dom {
    Shell::new()
    .title("RMComponents — Example page")
            .sidebar(vec![
                SidebarItem::Title("Components"),
                SidebarItem::Item("Button", "#button"),
                SidebarItem::Item("Text", "#text"),
                SidebarItem::Item("Flex", "#flex"),
            ])
            .child(Some(
                html!("div", {
                    .child(row("Button", vec![
                        display_case(Button::new().text("Hello, world!").dom(), "Button (Solid)"),
                        display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Light).dom(), "Button (Light)"),
                        display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Outline).dom(), "Button (Outline)"),
                        display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Subtle).dom(), "Button (Subtle)"),
                    ]))
                    .child(row("Text", vec![
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::Subscript).dom(), "Text (Subcript)"),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::Superscript).dom(), "Text (Superscript)"),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::Small).dom(), "Text (Small)"),
                        display_case(Text::new().text("Hello, world!").dom(), "Text (Default)"),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H4).dom(), "Text (H4)"),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H3).dom(), "Text (H3)"),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H2).dom(), "Text (H2)"),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H1).dom(), "Text (H1)"),
                    ]))
                    .child(row("Flex", vec![
                        display_case(
                            Flex::new()
                                .child(Button::new().colour(Colour::Orange).variant(ButtonVariant::Light).text("Hello,").dom())
                                .child(Button::new().colour(Colour::Orange).variant(ButtonVariant::Light).text("world!").dom())
                                .space_x(RemSizing::Md)
                                .space_y(RemSizing::None)
                                .dom(),
                            "Flex (Default)")
                    ]))
                })
            )).dom()
}

pub struct App {}
impl App {
    pub fn render(self: Rc<Self>) -> Dom {
        let mut provider = Provider::new();
        provider.child(display());
        provider.dom()
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    let model = Rc::new(App {});
    dominator::append_dom(&dominator::body(), model.render());
}
