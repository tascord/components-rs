use std::borrow::BorrowMut;

use dominator::{dom_builder, Dom, DomBuilder};
use std::hash::Hash;
use web_sys::HtmlElement;

use crate::helpers::{css::CSS, theme::THEME};

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
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
    None,
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Rem(f32),
}

impl PartialEq for RemSizing {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RemSizing::None, RemSizing::None) => true,
            (RemSizing::Xs, RemSizing::Xs) => true,
            (RemSizing::Sm, RemSizing::Sm) => true,
            (RemSizing::Md, RemSizing::Md) => true,
            (RemSizing::Lg, RemSizing::Lg) => true,
            (RemSizing::Xl, RemSizing::Xl) => true,
            (RemSizing::Rem(a), RemSizing::Rem(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for RemSizing {}
impl Hash for RemSizing {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            RemSizing::None => 0.hash(state),
            RemSizing::Xs => 1.hash(state),
            RemSizing::Sm => 2.hash(state),
            RemSizing::Md => 3.hash(state),
            RemSizing::Lg => 4.hash(state),
            RemSizing::Xl => 5.hash(state),
            RemSizing::Rem(rem) => rem.to_bits().hash(state),
        }
    }
}

pub trait Component {
    // Helper
    fn style(&mut self, style: (String, String)) -> &mut Self;

    // Constructing functions
    fn mt(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-".to_string(), s.to_string()))
    }
    fn mb(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-".to_string(), s.to_string()))
    }
    fn ml(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-".to_string(), s.to_string()))
    }
    fn mr(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-".to_string(), s.to_string()))
    }

    // Computing functions
    fn render(&mut self, id: String) -> Dom;
    fn css(&self) -> CSS;
    fn dom(&mut self) -> Dom {
        self.render(self.css().generate())
    }
    fn m_dom(&mut self) -> &mut Dom {
        // let mut dom = self.render(self.css().generate());
        // dom.borrow_mut()
        todo!()
    }
}

impl ToString for Colour {
    fn to_string(&self) -> String {
        let colours = THEME.get_cloned().colours.unwrap();
        match self {
            Colour::Hex(hex) => hex.to_string(),
            _ => colours.get(self).unwrap().to_string(),
        }
    }
}

impl ToString for TextColour {
    fn to_string(&self) -> String {
        match self {
            TextColour::Light => "#ffffff".to_string(),
            TextColour::Dark => "#000000".to_string(),
            TextColour::Accent => Colour::Blue.to_string(),
            TextColour::Hex(hex) => hex.to_string(),
        }
    }
}

impl ToString for RemSizing {
    fn to_string(&self) -> String {
        let sizing = THEME.get_cloned().sizing.unwrap();
        match self {
            RemSizing::None => "{}rem".to_string(),
            RemSizing::Rem(rem) => format!("{}rem", rem),
            _ => format!("{}rem", sizing.get(self).unwrap()),
        }
    }
}

impl RemSizing {
    pub fn mult(&self, value: f32) -> Self {
        match self {
            RemSizing::None => RemSizing::None,
            RemSizing::Xs => RemSizing::Rem(0.5 * value),
            RemSizing::Sm => RemSizing::Rem(0.75 * value),
            RemSizing::Md => RemSizing::Rem(value),
            RemSizing::Lg => RemSizing::Rem(1.5 * value),
            RemSizing::Xl => RemSizing::Rem(2.0 * value),
            RemSizing::Rem(rem) => RemSizing::Rem(rem * value),
        }
    }
}
