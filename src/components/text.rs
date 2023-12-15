use dominator::html;
use factoryizer::Factory;

use crate::helpers::css::{State, CSS};

use super::ty::{Component, TextColour};

#[derive(Default, Clone)]
pub enum TextVariant {
    Subscript,
    Superscript,
    Small,
    #[default]
    Default,
    H4,
    H3,
    H2,
    H1,
}

#[derive(Factory, Default)]
pub struct Text {
    text: &'static str,
    variant: TextVariant,
    colour: TextColour,
}

impl Component for Text {
    fn render(&mut self, id: String) -> dominator::Dom {
        html!({
            // This could all be a <span> but
            // samantics are important for accesibility
            // which I am aiming to make a priority
            match self.variant {
                TextVariant::Subscript => "sub",
                TextVariant::Superscript => "sup",
                TextVariant::Small | TextVariant::Default => "span",
                TextVariant::H4 => "h4",
                TextVariant::H3 => "h3",
                TextVariant::H2 => "h2",
                TextVariant::H1 => "h1"
            }
        }, {
            .attr("id", &id)
            .text(&self.text)
        })
    }
    fn css(&self) -> CSS {
        let c = CSS::new()
            .add_state(
                None,
                State::new()
                    .add_property("color", &self.colour.to_string())
                    .add_property("font-size", &match self.variant {
                        TextVariant::Subscript => "0.83em",
                        TextVariant::Superscript => "0.83em",
                        TextVariant::Small => "0.75rem",
                        TextVariant::Default => "1rem",
                        TextVariant::H4 => "1.5rem",
                        TextVariant::H3 => "2rem",
                        TextVariant::H2 => "3rem",
                        TextVariant::H1 => "4.5rem",
                    })
                    .add_property("font-weight", &match self.variant {
                        TextVariant::Subscript => "400",
                        TextVariant::Superscript => "400",
                        TextVariant::Small => "400",
                        TextVariant::Default => "400",
                        TextVariant::H4 => "600",
                        TextVariant::H3 => "700",
                        TextVariant::H2 => "700",
                        TextVariant::H1 => "700",
                    })
                    .clone(),
            )
            .clone();
        c
    }
}
