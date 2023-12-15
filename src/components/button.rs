use dominator::html;
use factoryizer::Factory;

use crate::helpers::{
    colours::{bw_on_bg, darken, opacity, TRANSPARENT},
    css::{State, CSS},
};

use super::ty::{Colour, Component, RemSizing};

#[derive(Default, Clone)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Light,
    Outline,
    Subtle,
}

#[derive(Factory, Default)]
pub struct Button {
    text: &'static str,
    variant: ButtonVariant,
    colour: Colour,
    size: RemSizing,
    radius: RemSizing,
    padding: RemSizing,
}

impl Component for Button {
    fn render(&mut self, id: String) -> dominator::Dom {
        html!("button", {
            .attr("id", &id)
            .text(&self.text)
        })
    }
    fn css(&self) -> CSS {
        let c = CSS::new()
            .add_state(
                None,
                State::new()
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
                Some("hover"),
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
                Some("active"),
                State::new().add_property("transform-y", "-0.5rem").clone(),
            )
            .clone();

        c
    }
}
