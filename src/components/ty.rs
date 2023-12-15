use dominator::{Dom, DomBuilder, dom_builder};
use web_sys::HtmlElement;

use crate::helpers::css::CSS;

#[derive(Default, Clone, Debug)]
pub enum Colour {
    Grey,
    #[default]
    Blue,
    Red,
    Pink,
    Orange,
    Hex(&'static str),
}

#[derive(Default, Clone, Debug)]
pub enum TextColour {
    Light,
    #[default]
    Dark,
    Accent,
    Hex(&'static str),
}

#[derive(Default, Clone, Debug)]
pub enum RemSizing {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Rem(f32),
}

pub trait Component {
    fn render(&mut self, id: String) -> Dom;
    fn css(&self) -> CSS;
    fn ok(&mut self) -> Dom {
        self.render(self.css().generate())
    }
    fn builder(&mut self) -> DomBuilder<HtmlElement> {
        dom_builder!(self.render(self.css().generate() as HtmlElement), {})
    }
}

impl ToString for Colour {
    fn to_string(&self) -> String {
        match self {
            Colour::Grey => "#9e9e9e",
            Colour::Blue => "#2196f3",
            Colour::Red => "#f44336",
            Colour::Pink => "#e91e63",
            Colour::Orange => "#ff9800",
            Colour::Hex(hex) => hex,
        }
        .to_string()
    }
}

impl ToString for TextColour {
    fn to_string(&self) -> String {
        match self {
            TextColour::Light => "#ffffff",
            TextColour::Dark => "#000000",
            TextColour::Accent => "#ff9800",
            TextColour::Hex(hex) => hex,
        }
        .to_string()
    }
}

impl ToString for RemSizing {
    fn to_string(&self) -> String {
        match self {
            RemSizing::Xs => "0.5rem".to_string(),
            RemSizing::Sm => "0.75rem".to_string(),
            RemSizing::Md => "1rem".to_string(),
            RemSizing::Lg => "1.5rem".to_string(),
            RemSizing::Xl => "2rem".to_string(),
            RemSizing::Rem(rem) => format!("{}rem", rem),
        }
    }
}

impl RemSizing {
    pub fn mult(&self, value: f32) -> Self {
        match self {
            RemSizing::Xs => RemSizing::Rem(0.5 * value),
            RemSizing::Sm => RemSizing::Rem(0.75 * value),
            RemSizing::Md => RemSizing::Rem(value),
            RemSizing::Lg => RemSizing::Rem(1.5 * value),
            RemSizing::Xl => RemSizing::Rem(2.0 * value),
            RemSizing::Rem(rem) => RemSizing::Rem(rem * value),
        }
    }
}
