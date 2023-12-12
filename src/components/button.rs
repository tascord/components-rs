use derive_builder::Builder;
use dominator::html;

use super::ty::{Colour, RemSizing, Component};

#[derive(Default, Clone)]
pub enum ButtonVariant {
    #[default]
    Filled,
    Light,
    Subtle,
}

#[derive(Builder, Default, Clone)]
pub struct Button {
    text: String,
    variant: ButtonVariant,
    colour: Colour,
    size: RemSizing,
    radius: RemSizing,
}

impl Component for Button {
    fn render(self) -> dominator::Dom {
        html!("button", {
            .text(&self.text)
            
        })
    }
}