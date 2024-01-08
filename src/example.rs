use crate::components::{
    button::ButtonVariant,
    tabs::Tab,
    text::TextVariant,
    tooltip::Tooltip,
    ty::{Colour, RemSizing},
    *,
};
use crate::components::{
    shell::SidebarItem,
    table::{numerical_sort, Table, TableValues},
};
use dominator::{class, html, with_node, Dom};
use crate::helpers::Provider;
use std::rc::Rc;
use tabler_dominator::icon;
use wasm_bindgen::prelude::*;

pub fn display_case(element: Dom, label: &str, tooltip: Option<&str>) -> Dom {
    html!("div", {
        .class(
            class! {
                .style("margin", "1rem")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("align-items", "center")
                .style("justify-content", "center")
                .style("border", "1px solid #eeeeee")
                .style("width", "20vw")
                .style("min-width", "20rem")
            }
        )
        .child(html!("div", {
            .class(
                class! {
                    .style("margin", "1rem")
                }
            )
            .child(element)
        }))
        .child(
            html!("h2", {
                .text(label)
                .class("flex")
                .class("items-center")
                .class("justify-center")
                .class("w-full")
                .class("py-1")
                .class("text-lg")
                .class(
                    class! {
                        .style("border-top", "1px solid #eeeeee")
                    }
                )
                .apply(|mut d| {
                    if tooltip.is_some() {
                        d = d.child(
                            Tooltip::new()
                                .child(icon!("info-circle"))
                                .text(tooltip.unwrap())
                                .ml(RemSizing::Md)
                                .dom()
                        )
                    }

                    d
                })
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
    let table_data = vec![
        TableValues::new()
            .title("Max Temp".to_string())
            .value("1")
            .value("2")
            .value("3")
            .sort(Some(numerical_sort))
            .clone(),
        TableValues::new()
            .title("Min Temp".to_string())
            .value("6")
            .value("5")
            .value("4")
            .sort(Some(numerical_sort))
            .clone(),
    ];

    Shell::new()
    .title("Components — Example page")
            .sidebar(vec![
                SidebarItem::Title("Components"),
                SidebarItem::Item("Tables", "#table"),
                SidebarItem::Item("Tabs", "#tabs"),
                SidebarItem::Item("Buttons", "#button"),
                SidebarItem::Item("Text", "#text"),
                SidebarItem::Item("Utils", "#util"),
            ])
            .child(Some(
                html!("div", {
                    .child(row("Inputs", vec![
                        display_case(SegmentedControl::new().option(("Read Only", "ro")).option(("Read/Write", "rw")).default_value("rw").dom(), "Segmented Control (Default)", None),
                        display_case(SegmentedControl::new().option(("Read Only", "ro")).option(("Read/Write", "rw")).default_value("rw").colour(Colour::Pink).dom(), "Segmented Control (Pink)", None),
                        display_case(SegmentedControl::new().option(("Read Only", "ro")).option(("Read/Write", "rw")).default_value("rw").colour(Colour::Coral).dom(), "Segmented Control (Coral)", None),
                        display_case(SegmentedControl::new().option(("Read Only", "ro")).option(("Read/Write", "rw")).default_value("rw").colour(Colour::Grey).dom(), "Segmented Control (Grey)", None)
                    ]))
                    .child(row("Table", vec![
                        display_case(
                            Table::new().data(table_data.clone().into()).direction(TableDirection::Column).dom(),
                            "Table (Default)", None
                        ),
                        display_case(
                            Table::new().data(table_data.clone().into()).direction(TableDirection::Column).colour(Colour::Pink).dom(),
                            "Table (Pink)", None
                        ),
                        display_case(
                            Table::new().data(table_data.clone().into()).direction(TableDirection::Column).colour(Colour::Coral).dom(),
                            "Table (Coral)", None
                        ),
                        display_case(
                            Table::new().data(table_data.clone().into()).direction(TableDirection::Column).colour(Colour::Grey).dom(),
                            "Table (Grey)", None
                        ),
                    ]))
                    .child(row("Tabs", vec![
                        display_case(Tabs::new().placement(tabs::TabPlacement::Bottom).tabs(vec![
                            Tab::new("Source Code", "source", html!("pre", { .text("<rust>") })),
                            Tab::new("Generated Html", "generated", html!("pre", { .text("<html>") }))
                        ]).dom(), "Tabs (Bottom)", None),
                        display_case(Tabs::new().colour(Colour::Pink).placement(tabs::TabPlacement::Top).tabs(vec![
                            Tab::new("Source Code", "source", html!("pre", { .text("<rust>") })),
                            Tab::new("Generated Html", "generated", html!("pre", { .text("<html>") }))
                        ]).dom(), "Tabs (Top, Pink)", None),
                        display_case(Tabs::new().colour(Colour::Coral).placement(tabs::TabPlacement::Left).tabs(vec![
                            Tab::new("Source Code", "source", html!("pre", { .text("<rust>") })),
                            Tab::new("Generated Html", "generated", html!("pre", { .text("<html>") }))
                        ]).dom(), "Tabs (Left, Coral)", None),
                        display_case(Tabs::new().colour(Colour::Grey).placement(tabs::TabPlacement::Right).tabs(vec![
                            Tab::new("Source Code", "source", html!("pre", { .text("<rust>") })),
                            Tab::new("Generated Html", "generated", html!("pre", { .text("<html>") }))
                        ]).dom(), "Tabs (Right, Grey)", None),
                    ]))
                    .child(row("Button", vec![
                        display_case(Button::new().text("Hello, world!").dom(), "Button (Solid, Blue)", None),
                        display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Light).colour(Colour::Pink).dom(), "Button (Light, Pink)", None),
                        display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Outline).colour(Colour::Coral).dom(), "Button (Outline, Coral)", None),
                        display_case(Button::new().text("Hello, world!").variant(ButtonVariant::Subtle).colour(Colour::Grey).dom(), "Button (Subtle, Grey)", None),
                    ]))
                    .child(row("Text", vec![
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::Subscript).dom(), "Text (Subcript)", None),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::Superscript).dom(), "Text (Superscript)", None),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::Small).dom(), "Text (Small)", None),
                        display_case(Text::new().text("Hello, world!").dom(), "Text (Default)", None),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H4).dom(), "Text (H4)", None),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H3).dom(), "Text (H3)", None),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H2).dom(), "Text (H2)", None),
                        display_case(Text::new().text("Hello, world!").variant(TextVariant::H1).dom(), "Text (H1)", None),
                    ]))
                    .child(row("Util", vec![
                        display_case(
                            Flex::new()
                                .child(Button::new().colour(Colour::Coral).variant(ButtonVariant::Light).text("Hello,").dom())
                                .child(Button::new().colour(Colour::Coral).variant(ButtonVariant::Light).text("world!").dom())
                                .space_x(RemSizing::Md)
                                .space_y(RemSizing::None)
                                .dom(),
                            "Flex", Some("Flex is a component that allows you to easily space elements in a row or column.")
                        ),
                        display_case(
                            Tooltip::new()
                                .child(Button::new().colour(Colour::Blue).variant(ButtonVariant::Light).text("Hover me!").dom())
                                .text("Look at me, I'm a tooltip!")
                                .dom(),
                            "Tooltip", None
                        )
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
