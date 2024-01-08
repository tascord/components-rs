use std::collections::HashMap;

use dominator::{html, Dom};
use factoryizer::Factory;

use crate::helpers::safety::none_dom;

use super::{ty::Reactive, Component};

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub enum Position {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

lazy_static::lazy_static! {
    static ref POS_STYLES: HashMap<Position, Vec<String>> = {
        let mut m = HashMap::new();
        m.insert(Position::Top, vec!["bottom-[120%]", "origin-bottom", "after:rotate-180", "after:bottom-[-0.85rem]", "after:left-[50%]", "after:translate-x-[-7.5px]"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
        m.insert(Position::Bottom, vec!["-bottom-[120%]", "origin-top", "after:rotate-0", "after:top-[-0.85rem]", "after:left-[50%]", "after:translate-x-[-7.5px]"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
        m.insert(Position::Left, vec!["right-[120%]", "origin-right", "after:rotate-90", "after:right-[-0.9rem]"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
        m.insert(Position::Right, vec!["-right-[120%]", "origin-left", "after:-rotate-90", "after:left-[-0.9rem]"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
        m
    };
}

#[derive(Factory, Default)]
pub struct Tooltip {
    position: Position,

    #[skip]
    child: Option<Dom>,
    #[skip]
    text: String,
    #[skip]
    styles: Vec<(String, Reactive<String>)>,
}

impl Tooltip {
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.text = text.to_string();
        self
    }
    pub fn child(&mut self, child: Dom) -> &mut Self {
        self.child = Some(child);
        self
    }
}

impl Component for Tooltip {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }

    fn dom(&mut self) -> Dom {
        html!("div", {
            .class("relative")
            .class("grid")
            .class("place-items-center")
            .class("group")
            .child(self.child.take().unwrap_or_else(|| none_dom()))
            .apply(|mut d| {
                for (k, v) in self.styles.iter() {
                    d = v.apply_style(k.to_string(), d);
                }

                d
            })
            .child(html!("span", {

                // Layout
                .class("absolute")
                .class("w-auto")
                .class("min-w-max")
                .class("p-2")
                .class("mx-2")
                .class("rounded-md")
                .class("shadow-md")

                // Text
                .class("text-white")
                .class("bg-neutral-700")
                .class("text-xs")
                .class("font-bold")

                // Transition
                .class("transition-all")
                .class("duration-[120]")
                .class("ease-out")
                .class("group-hover:scale-100")
                .class("transform-gpu")

                // Original state
                .class("scale-0")

                // Arrow
                .class("after:absolute")
                .class("after:border-[7.5px]")
                .class("after:border-transparent")
                .class("after:border-b-neutral-700")

                // Content
                .text(&self.text)

                .apply(|mut d| {
                    // TODO: You can do this with multistr im liek 99% sure
                    for style in POS_STYLES.get(&self.position).unwrap_or(&vec![]) {
                        d = d.class(style);
                    }

                    d
                })
            }))
        })
    }
}
