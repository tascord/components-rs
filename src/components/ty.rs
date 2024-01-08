use dominator::{Dom, DomBuilder};
use futures_signals::signal::Mutable;
use std::hash::Hash;
use web_sys::HtmlElement;

use crate::helpers::theme::THEME;

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Colour {
    Grey,
    #[default]
    Blue,
    Coral,
    Pink,
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
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self;

    // Constructing functions
    fn mt(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-top".to_string(), s.to_string().into()))
    }
    fn mb(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-bottom".to_string(), s.to_string().into()))
    }
    fn ml(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-left".to_string(), s.to_string().into()))
    }
    fn mr(&mut self, s: RemSizing) -> &mut Self {
        self.style(("margin-right".to_string(), s.to_string().into()))
    }

    // Computing functions
    fn dom(&mut self) -> Dom;
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

#[derive(Clone)]
pub enum Reactive<T> {
    Static(T),
    Dynamic(Mutable<T>),
}

impl Default for Reactive<String> {
    fn default() -> Self {
        Reactive::Static("".to_string())
    }
}

impl Default for Reactive<&'static str> {
    fn default() -> Self {
        Reactive::Static("")
    }
}

// Style/Class implimentations
impl From<String> for Reactive<String> {
    fn from(value: String) -> Self {
        Reactive::Static(value)
    }
}

impl From<Mutable<String>> for Reactive<String> {
    fn from(value: Mutable<String>) -> Self {
        Reactive::Dynamic(value)
    }
}

impl Reactive<String> {
    pub fn apply_style(&self, name: String, e: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        match self {
            Reactive::Static(value) => e.style(name, value),
            Reactive::Dynamic(value) => e.style_signal(name, value.signal_cloned()),
        }
    }
    pub fn apply_text(&self, e: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        match self {
            Reactive::Static(value) => e.text(value),
            Reactive::Dynamic(value) => e.text_signal(value.signal_cloned()),
        }
    }
}

// Text implimentations
impl From<&'static str> for Reactive<String> {
    fn from(value: &'static str) -> Self {
        Reactive::Static(value.to_string())
    }
}
