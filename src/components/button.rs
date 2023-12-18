use std::rc::Rc;

use dominator::{html, with_node};
use factoryizer::Factory;

use crate::helpers::{
    colours::{bw_on_bg, darken, opacity, TRANSPARENT},
    css::{State, CSS},
};

use super::ty::{Colour, Component, Reactive, RemSizing};

#[derive(Default, Clone)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Light,
    Outline,
    Subtle,
}

#[derive(Factory, Default)]
#[into]
pub struct Button {
    text: Reactive<&'static str>,
    variant: ButtonVariant,
    colour: Colour,
    size: RemSizing,
    radius: RemSizing,
    padding: RemSizing,
    #[skip]
    on_click: Option<Rc<dyn Fn()>>,
    #[skip]
    styles: Vec<(String, Reactive<String>)>,
}

impl Button {
    pub fn on_click(&mut self, closure: impl Fn() + 'static) -> &mut Self {
        self.on_click = Some(Rc::new(closure));
        self
    }  
}

impl Component for Button {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }

    fn render(&mut self, class: String) -> dominator::Dom {
        html!("button", {
            .class(&class)
            .with_node!(_e => {
                .event({
                    let on_click = self.on_click.clone().unwrap_or(Rc::new(|| {}));
                    move |_evt: dominator::events::Click| {
                        on_click();
                    }
                })
            })
            .apply(|mut d| {
                d = self.text.apply(d);
                for (k, v) in self.styles.iter() {
                    d = v.apply(k.to_string(), d);
                }
                d
            })
        })
    }
    
    fn css(&self) -> CSS {
        let c = CSS::new()
            .add_state(
                None,
                State::new()
                    // .bulk(&self.styles)
                    .add_property("appearance", "none")
                    .add_property("border", "none")
                    .add_property("border-radius", &self.radius.mult(0.45).to_string())
                    .add_property("cursor", "pointer")
                    .add_property("font-weight", "600")
                    .add_property("font-size", &self.size.to_string())
                    .add_property(
                        "outline",
                        &match self.variant {
                            ButtonVariant::Outline => {
                                format!("2px solid {}", self.colour.to_string())
                            }
                            _ => "none".to_string(),
                        },
                    )
                    .add_property(
                        "background",
                        &match self.variant {
                            ButtonVariant::Filled => self.colour.to_string(),
                            ButtonVariant::Light => opacity(self.colour.to_string(), 0.15),
                            ButtonVariant::Subtle | ButtonVariant::Outline => {
                                TRANSPARENT.to_string()
                            }
                        },
                    )
                    .add_property(
                        "color",
                        &match self.variant {
                            ButtonVariant::Filled => bw_on_bg(self.colour.to_string()),
                            ButtonVariant::Light
                            | ButtonVariant::Subtle
                            | ButtonVariant::Outline => self.colour.to_string(),
                        },
                    )
                    .add_property(
                        "padding",
                        &format!(
                            "{} {}",
                            self.padding.mult(0.5).to_string(),
                            self.padding.to_string()
                        ),
                    )
                    .clone(),
            )
            .add_state(
                Some(":hover"),
                State::new()
                    .add_property(
                        "background",
                        &match self.variant {
                            ButtonVariant::Filled => darken(self.colour.to_string(), 0.25),
                            ButtonVariant::Light => opacity(self.colour.to_string(), 0.25),
                            ButtonVariant::Subtle | ButtonVariant::Outline => {
                                opacity(self.colour.to_string(), 0.15)
                            }
                        },
                    )
                    .clone(),
            )
            .add_state(
                Some(":active"),
                State::new().add_property("transform-y", "-0.5rem").clone(),
            )
            .clone();

        c
    }
}
