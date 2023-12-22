use dominator::{html, with_node, Dom};
use factoryizer::Factory;
use futures_signals::signal::{Mutable, SignalExt};
use tabler_dominator::icon;

use crate::components::{
    brand::{Brand, BrandVariant},
    button::ButtonVariant,
    Button,
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
    Default,
}

impl Component for Shell {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn render(&mut self, _: String) -> dominator::Dom {
        let open = self.open.clone();
        html!("div", {
            .class("flex")
            .class("flex-col")
            .class("w-screen")
            .class("h-screen")
            .class("fixed")
            .style_signal("--sidebar-width", self.open.signal_cloned().map(|o| {
                if o == SidebarState::Open { "var(--sidebar-open-width)" } else { "0" }
            }))
            .child(
                html!("header", {
                    .class("text-white")
                    .class("w-full")
                    .class("h-[var(--title-height)]")
                    .class("bg-black")
                    .class("flex")
                    .class("items-center")
                    .class("space-x-2")
                    .child_signal(open.signal_cloned().map(move |o| {
                        let open = open.clone();
                        match o {
                            SidebarState::Open => None,
                            SidebarState::Closed | SidebarState::Default => Some(
                                Button::new().variant(ButtonVariant::Subtle).child(icon!("menu-2")).on_click(move || {
                                    open.set(SidebarState::Open)
                                }).dom()
                            )
                        }
                    }))
                    .child(
                        html!("a", {
                            .class("border-0")
                            .class("w-[max(var(--sidebar-width),7.5rem)]")
                            .class("h-[var(--title-height)]")
                            .class("grid")
                            .class("place-items-center")
                            .child(
                                Brand::new().variant(BrandVariant::Mark).style(("height".to_string(), "calc(100% - 1rem)".to_string().into())).dom()
                            )
                            .attr("href", "/")
                        })
                    )
                    .child(
                        html!("h1", {
                            .text(&self.title)
                            .class("text-xl")
                            .class("font-semibold")
                        })
                    )
                })
            )
            .child(
                html!("nav", {
                    .class("text-white")
                    .class("flex")
                    .class("flex-col")
                    .class("flex-1")
                    .class("w-[var(--sidebar-width)]")
                    .class("max-w-[var(--sidebar-width)]")
                    .class("items-center")
                    .class("bg-black")
                    .class("overflow-y-auto")
                    .class("overflow-x-hidden")
                    .child(html!("ul", {
                        .class("w-full")
                        .class("overflow-x-auto") // TODO: Make this elipsis
                        .children(
                            self.sidebar.iter_mut().map(|item| {
                                match item {
                                    SidebarItem::Item(text, link) => {
                                       html!("li", {
                                            .child( html!("a", {
                                                .text(text)
                                                .attr("href", link)
                                                .class("pl-2")
                                                .class("mb-1")
                                                .class("w-[calc(100%-1rem)]")
                                            }))
                                       })
                                    }
                                    SidebarItem::Title(text) => {
                                        html!("h2", {
                                            .text(text)
                                            .class("text-center")
                                            .class("my-2")
                                        })
                                    }
                                    SidebarItem::Spacer => {
                                        html!("hr", {})
                                    }
                                }
                            })
                        )
                    }))
                    .child(Button::new().variant(ButtonVariant::Subtle).child(icon!("x")).on_click({
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
                    .class("absolute")
                    .class("left-0")
                    .class("top-0")
                    .class("ml-[var(--sidebar-width)]")
                    .class("mt-[var(--title-height)]")
                    .class("overflow-auto")
                    .class("w-[calc(100vw-var(--sidebar-width))]")
                    .class("h-[calc(100vh-var(--title-height))]")
                    .class("max-w-[calc(100vw-var(--sidebar-width))]")
                    .class("max-h-[calc(100vh-var(--title-height))]")
                })
            )
        })
    }
    fn css(&self) -> crate::helpers::css::CSS {
        crate::helpers::css::CSS::new()
    }
}
