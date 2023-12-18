use dominator::{html, Dom};
use factoryizer::Factory;
use futures_signals::signal::{Mutable, SignalExt};

use crate::{
    components::{
        brand::{Brand, BrandVariant},
        button::ButtonVariant,
        ty::RemSizing,
        Button,
    },
    helpers::css::{State, CSS},
};

use super::ty::{Component, Reactive};

pub enum SidebarItem {
    Item(&'static str, &'static str),
    Title(&'static str),
    Spacer,
}

#[derive(Factory, Default)]
pub struct Shell {
    child: Option<Dom>,
    title: &'static str,
    sidebar: Vec<SidebarItem>,

    styles: Vec<(String, Reactive<String>)>,
    open: Mutable<SidebarState>,
}

#[derive(Default, Clone, PartialEq)]
pub enum SidebarState {
    #[default]
    Open,
    Closed,
}

impl Component for Shell {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn render(&mut self, class: String) -> dominator::Dom {
        let open = self.open.clone();
        html!("div", {
            .class(&class)
            .style_signal("--sidebar-width", self.open.signal_cloned().map(|o| {
                if o == SidebarState::Open { "var(--sidebar-open-width)" } else { "0" }
            }))
            .child(
                html!("header", {
                    .child_signal(open.signal_cloned().map(move |o| {
                        let open = open.clone();
                        match o {
                            SidebarState::Open => None,
                            SidebarState::Closed => Some(
                                Button::new().variant(ButtonVariant::Light).text("+").on_click(move || {
                                    open.set(SidebarState::Open)
                                }).dom()
                            )
                        }
                    }))
                    .child(
                        html!("a", {
                            .child(
                                Brand::new().variant(BrandVariant::Mark).style(("height".to_string(), "calc(100% - 1rem)".to_string().into())).dom()
                            )
                            .attr("href", "/")
                        })
                    )
                    .child(
                        html!("h1", {
                            .text(&self.title)
                        })
                    )
                })
            )
            .child(
                html!("nav", {
                    .child(html!("ul", {
                        .children(
                            self.sidebar.iter_mut().map(|item| {
                                match item {
                                    SidebarItem::Item(text, link) => {
                                       html!("li", {
                                            .child( html!("a", {
                                                .text(text)
                                                .attr("href", link)
                                            }))
                                       })
                                    }
                                    SidebarItem::Title(text) => {
                                        html!("h2", {
                                            .text(text)
                                        })
                                    }
                                    SidebarItem::Spacer => {
                                        html!("hr", {})
                                    }
                                }
                            })
                        )
                    }))
                    .child(Button::new().variant(ButtonVariant::Subtle).text("-").on_click({
                        let open = self.open.clone();
                        move || {
                            open.set({
                                if open.get_cloned() == SidebarState::Open {
                                    SidebarState::Closed
                                } else {
                                    SidebarState::Open
                                }
                            });
                        }
                    }).dom())
                }
            ))
            .child(
                html!("main", {
                    .child(self.child.take().unwrap())
                })
            )
        })
    }
    fn css(&self) -> CSS {
        let c = CSS::new()
            .add_state(
                None,
                State::new()
                    .add_property("position", "fixed")
                    .add_property("display", "flex")
                    .add_property("flex-direction", "column")
                    .add_property("width", "100vw")
                    .add_property("height", "100vh")
                    .clone(),
            )
            .add_state(
                Some("> header"),
                State::new()
                    .add_property("color", "white")
                    .add_property("width", "100%")
                    .add_property("height", "var(--title-height)")
                    .add_property("background", "black")
                    .add_property("display", "flex")
                    .add_property("align-items", "center")
                    .clone(),
            )
            .add_state(
                Some("> header > a"),
                State::new()
                    .add_property("width", "max(var(--sidebar-width), 7.5rem)")
                    .add_property("height", "var(--title-height)")
                    .add_property("display", "grid")
                    .add_property("place-items", "center")
                    .clone(),
            )
            .add_state(
                Some("> header > h1"),
                State::new()
                    .add_property("margin", "0")
                    .add_property("padding", "0")
                    .add_property("font-size", "1.25rem")
                    .add_property("font-weight", "400")
                    .clone(),
            )
            .add_state(
                Some("> header > *"),
                State::new().add_property("margin", "0.5rem").clone(),
            )
            .add_state(
                Some("> nav"),
                State::new()
                    .add_property("color", "white")
                    .add_property("display", "flex")
                    .add_property("flex-direction", "column")
                    .add_property("flex", "1")
                    .add_property("width", "var(--sidebar-width)")
                    .add_property("max-width", "var(--sidebar-width)")
                    .add_property("background", "black")
                    .add_property("align-items", "center")
                    .add_property("overflow-y", "auto")
                    .add_property("overflow-x", "clip")
                    .clone(),
            )
            .add_state(
                Some("> nav > ul"),
                State::new()
                    .add_property("padding", "0")
                    .add_property("margin", "0")
                    .add_property("list-style", "none")
                    .add_property("width", "100%")
                    .add_property("overflow-x", "auto")
                    .clone(),
            )
            .add_state(
                Some("> nav > ul > li"),
                State::new()
                    .add_property("padding-left", "1rem")
                    .add_property("margin-bottom", "0.25rem")
                    .add_property("width", "calc(100% - 1rem)")
                    .clone(),
            )
            .add_state(
                Some("> nav > ul > h2"),
                State::new()
                    .add_property("text-align", "center")
                    .add_property("margin", "1rem 0")
                    .clone(),
            )
            .add_state(
                Some("> main"),
                State::new()
                    .add_property("position", "absolute")
                    .add_property("left", "0")
                    .add_property("top", "0")
                    .add_property("margin-left", "var(--sidebar-width)")
                    .add_property("margin-top", "var(--title-height)")
                    .add_property("overflow", "auto")
                    .add_property("width", "calc(100vw - var(--sidebar-width))")
                    .add_property("height", "calc(100vh - var(--title-height))")
                    .add_property("max-width", "calc(100vw - var(--sidebar-width))")
                    .add_property("max-height", "calc(100vh - var(--title-height))")
                    .clone(),
            )
            .clone();
        c
    }
}
