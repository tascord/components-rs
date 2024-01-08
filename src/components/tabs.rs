use dominator::{class, html, pseudo, with_node, Dom};
use factoryizer::Factory;
use futures_signals::signal::{Mutable, SignalExt};

use crate::helpers::colours::{bw_on_bg, opacity};

use super::ty::{Colour, Component, Reactive};

#[derive(Default, Clone)]
pub enum TabPlacement {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

pub struct Tab {
    title: &'static str,
    id: &'static str,
    element: Dom,
}

impl Tab {
    pub fn new(title: &'static str, id: &'static str, element: Dom) -> Self {
        Self { title, id, element }
    }
}

#[derive(Factory, Default)]
pub struct Tabs {
    tabs: Vec<Tab>,
    styles: Vec<(String, Reactive<String>)>,
    placement: TabPlacement,
    colour: Colour,

    #[skip]
    pub selected: Mutable<String>,
}

impl Tabs {
    pub fn tab(&mut self, child: Tab) -> &mut Self {
        self.tabs.push(child);
        self
    }
}

impl Component for Tabs {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn dom(&mut self) -> dominator::Dom {
        if self.selected.get_cloned().is_empty() && self.tabs.len() > 0 {
            self.selected.set(self.tabs[0].id.to_string());
        }

        html!("div", {
            .class("flex")
            .class(match self.placement {
                TabPlacement::Top => "flex-col",
                TabPlacement::Bottom => "flex-col-reverse",
                TabPlacement::Left => "flex-row",
                TabPlacement::Right => "flex-row-reverse",
            })
            .child(html!("div", {
                .class("flex")
                .class(match self.placement {
                    TabPlacement::Top | TabPlacement::Bottom => "flex-row",
                    TabPlacement::Left | TabPlacement::Right => "flex-col",
                })
                .class(match self.placement {
                    TabPlacement::Top | TabPlacement::Bottom => "space-x-4",
                    TabPlacement::Left | TabPlacement::Right => "space-y-2",
                })
                .class(match self.placement {
                    TabPlacement::Top => "mb-2",
                    TabPlacement::Bottom => "mt-2",
                    TabPlacement::Left => "mr-2",
                    TabPlacement::Right => "ml-2",
                })
                .class(match self.placement {
                    TabPlacement::Top => "border-b",
                    TabPlacement::Bottom => "border-t",
                    TabPlacement::Left => "border-r",
                    TabPlacement::Right => "border-l",
                })
                .class("border-neutral-300")
                .apply(|mut d| {
                    let selected = self.selected.clone();
                    for tab in &self.tabs {
                        let selected = selected.clone();
                        let id = tab.id;

                        d = d.child(html!("button", {
                            .text(tab.title)
                            .class("font-semibold")
                            .class("px-4")
                            .class("py-2")
                            .class(match self.placement {
                                TabPlacement::Top => "rounded-t",
                                TabPlacement::Bottom => "rounded-b",
                                TabPlacement::Left => "rounded-l",
                                TabPlacement::Right => "rounded-r",
                            })
                            .class(
                                class! {
                                    .style_signal("color", selected.signal_cloned().map({
                                        let colour = self.colour.to_string();
                                        move |s| {
                                            if s == id {
                                                bw_on_bg(colour.clone())
                                            } else {
                                                colour.clone()
                                            }
                                        }
                                    }))
                                    .style_signal("background", selected.signal_cloned().map({
                                        let colour = self.colour.to_string();
                                        move |s| {
                                            if s == id {
                                                opacity(colour.clone(), 0.75)
                                            } else {
                                                "transparent".to_string()
                                            }
                                        }
                                    }))
                                    .pseudo!(":hover", {
                                        .style("color", bw_on_bg(self.colour.to_string()))
                                        .style_signal("background", selected.signal_cloned().map({
                                            let colour = self.colour.to_string();
                                            move |s| {
                                                if s != id {
                                                    opacity(colour.clone(), 0.5)
                                                } else {
                                                    opacity(colour.clone(), 0.75) // TODO: Can this just be blank?
                                                }
                                            }
                                        }))
                                    })
                                }
                            )
                            .with_node!(_e => {
                                .event({
                                    let selected = selected.clone();
                                    move |_evt: dominator::events::Click| {
                                            selected.clone().set(id.to_string());
                                    }
                                })
                            })
                        }));
                    }

                    d
                })
            }))
            .apply(|mut d| {
                for tab in self.tabs.drain(..) {
                    d = d.child(html!(
                        "div", {
                            .class_signal("hidden", self.selected.signal_ref(move |s| s != tab.id))
                            .child(tab.element)
                        }
                    ));
                }

                d
            })
        })
    }
}
