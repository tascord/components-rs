use std::fmt::format;

use dominator::{class, events, html, pseudo, with_node, Dom};
use factoryizer::Factory;
use futures_signals::signal::{Mutable, SignalExt};
use gloo_timers::callback::Timeout;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::{
    console_log,
    helpers::{
        colours::opacity,
        mutable::Mutable2,
        safety::{parent_or_self, rand_id},
    },
};

use super::{
    ty::{Colour, Reactive},
    Component,
};

#[derive(Factory, Default)]
pub struct SegmentedControl {
    colour: Colour,
    options: Vec<(String, String)>,
    pub value: Mutable<String>,
    #[skip]
    offset: Mutable<(i32, i32)>,
    #[skip]
    ready: Mutable<bool>,
    #[skip]
    styles: Vec<(String, Reactive<String>)>,
}

impl SegmentedControl {
    pub fn default_value(&mut self, value: &str) -> &mut Self {
        if self.value.get_cloned().is_empty() {
            self.value = Mutable::new(value.to_string());
        }
        self
    }
    pub fn option(&mut self, option: (&str, &str)) -> &mut Self {
        self.options
            .push((option.0.to_string(), option.1.to_string()));
        self
    }
}

impl Component for SegmentedControl {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }

    fn dom(&mut self) -> Dom {
        if self.value.get_cloned().is_empty() && self.options.len() > 0 {
            self.value = Mutable::new(self.options[0].0.clone());
        }

        html!("div", {
            .class("flex")
            .child(
                html!("div", {

                        // Container
                        .class("py-2")
                        .class("px-2")

                        // Controls
                        .class("relative")
                        .class("inline-flex")
                        .class("justify-between")
                        .class("text-xl")

                        // Segment (tw)
                        .class("border")
                        .class("border-neutral-200")
                        .class("rounded-lg")
                        .class("shadow-sm")

                        .class(
                            class! {
                                // Variables
                                .style_signal("--seg-x", self.offset.signal_cloned().map(|(v, _)| format!("{}px", v)))
                                .style_signal("--seg-w", self.offset.signal_cloned().map(|(_, v)| format!("{}px", v)))

                                // Segment
                                .pseudo!("::before", {
                                    .style("content", "''")
                                    .style("background", self.colour.to_string())
                                    .style("position", "absolute")
                                    .style("width", "var(--seg-w)")
                                    .style("transform", "translateX(var(--seg-x))")
                                    .style("top", "4px")
                                    .style("bottom", "4px")
                                    .style("left", "0")
                                    .style("z-index", "0")
                                    .style("border-radius", "0.5rem")
                                    .style_signal("transition", self.ready.signal_cloned().map(|ready| if ready {
                                        "transform 0.2s ease-in-out"
                                    } else {
                                        "none"
                                    }))
                                })
                            }
                        )

                        .apply({
                            let name = rand_id();
                            let options = self.options.clone();

                            move |mut d| {
                                for (k, v) in options.into_iter() {
                                    let id = rand_id();

                                    d = d.child(html!("div", {
                                        .class("relative")
                                        .class("z-[1]")
                                        .class("text-center")
                                        .class("px-4")

                                        .child(html!("input", {
                                            .prop("name", name.clone())
                                            .prop("id", id.clone())
                                            .prop("type", "radio")
                                            .prop_signal("checked", self.value.signal_cloned().map({
                                                let v = v.clone();
                                                move |value| value == v
                                            }))
                                            .with_node!(e => {
                                                .event({
                                                    let e = e.clone();
                                                    let offset_signal = self.offset.clone();
                                                    let value_signal = self.value.clone();
                                                    let own_value = v.clone();
                                                    move |_: events::Change| {
                                                        let e = parent_or_self(e.clone());
                                                        value_signal.set(own_value.clone());
                                                        offset_signal.set((e.offset_left(), e.offset_width()));
                                                    }
                                                })
                                                .after_inserted({
                                                    let offset_signal = self.offset.clone();
                                                    let ready_signal = self.ready.clone();
                                                    move |e| {
                                                    if e.dyn_ref::<HtmlInputElement>().unwrap().checked() {
                                                        // TODO: Use mutation observer instead
                                                        let t = Timeout::new(200, move || {
                                                            let e = parent_or_self(e.clone());
                                                            offset_signal.set((e.offset_left(), e.offset_width()));
                                                            let t = Timeout::new(200, move || {
                                                                ready_signal.set(true);
                                                            });
                                                            t.forget();
                                                        });
                                                        t.forget();
                                                    }
                                                }})
                                            })
                                            .class("opacity-0")
                                            .class("absolute")
                                            .class("top-0")
                                            .class("left-0")
                                            .class("cursor-pointer")
                                            .class("w-full")
                                            .class("h-full")
                                        }))
                                        .child(html!("label", {
                                            .text(&k)
                                            .attr("for", &id.clone())
                                            .class("user-select-none")
                                            .class("cursor-pointer")
                                            .class("max-w-[10rem]")
                                            .class("truncate")
                                            .class("transition")
                                            .class("font-semibold")
                                            .class_signal("text-white", self.value.signal_cloned().map({
                                                let v = v.clone();
                                                move |value| value == v
                                            }))
                                        }))
                                    }));
                                }

                                d
                            }
                        })
                    })
            )
        })
    }
}
