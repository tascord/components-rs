use dominator::{html, traits::StaticEvent, with_node};
use factoryizer::Factory;

use super::ty::{Component, Reactive, TextColour};
use crate::helpers::css::{State, CSS};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOOP: String =
        String::from_utf8_lossy(include_bytes!("../../assets/logo/Loop.svg")).to_string();
    static ref TEXT: String =
        String::from_utf8_lossy(include_bytes!("../../assets/logo/Text.svg")).to_string();
    static ref COMBINATION: String = format!("{}{}", &*LOOP, &*TEXT);
}

#[derive(Default, Clone)]
pub enum BrandVariant {
    Mark,
    Text,
    #[default]
    MarkText,
}

#[derive(Default, Clone)]
pub enum BrandColour {
    #[default]
    Light,
    Dark,
}

#[derive(Factory, Default)]
pub struct Brand {
    variant: BrandVariant,
    colour: BrandColour,

    styles: Vec<(String, Reactive<String>)>,
}

impl Component for Brand {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn render(&mut self, class: String) -> dominator::Dom {
        html!("div" => web_sys::HtmlElement, {
            .class(&class)
            .apply(|mut d| {
                for (k, v) in self.styles.iter() {
                    d = v.apply(k.to_string(), d)
                }
                d
            })
            .with_node!(element => {
                .text({
                    element.set_inner_html({
                        match self.variant {
                            BrandVariant::Mark => &*LOOP,
                            BrandVariant::Text => &*TEXT,
                            BrandVariant::MarkText => &*COMBINATION,
                        }
                    });
                    ""
                })
            })
        })
    }
    fn css(&self) -> CSS {
        let c = CSS::new()
            .add_state(
                None,
                State::new()
                    .add_property("display", "flex")
                    .add_property("flex-wrap", "nowrap")
                    .add_property("justify-content", "center")
                    .add_property("align-items", "center")
                    .add_property("height", "100%")
                    .add_property(
                        "fill",
                        match self.colour {
                            BrandColour::Light => "#fff",
                            BrandColour::Dark => "#000",
                        },
                    )
                    .clone(),
            )
            .add_state(
                Some(" svg"),
                State::new()
                    .add_property("height", "100%")
                    .add_property("margin", "auto")
                    .clone(),
            )
            .add_state(
                Some("> * + *"),
                State::new().add_property("margin-left", "0.5rem").clone(),
            )
            .clone();
        c
    }
}
