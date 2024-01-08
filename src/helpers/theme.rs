use std::collections::HashMap;

use dominator::{html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use lazy_static::lazy_static;

use crate::components::ty::{Colour, RemSizing};

lazy_static! {
    pub static ref THEME: Mutable<Theme> = Mutable::new(Theme::default());
}

#[derive(Clone)]
pub struct Theme {
    pub colours: Option<HashMap<Colour, String>>,
    pub sizing: Option<HashMap<RemSizing, f32>>,
}

impl Default for Theme {
    fn default() -> Self {
        let mut colours: HashMap<Colour, String> = HashMap::new();
        colours.insert(Colour::Grey, "#1E1E1E".to_string());
        colours.insert(Colour::Blue, "#5576B9".to_string());
        colours.insert(Colour::Coral, "#F15A4D".to_string());
        colours.insert(Colour::Pink, "#EE3075".to_string());

        let mut sizing: HashMap<RemSizing, f32> = HashMap::new();
        sizing.insert(RemSizing::Xs, 0.5);
        sizing.insert(RemSizing::Sm, 0.75);
        sizing.insert(RemSizing::Md, 1.0);
        sizing.insert(RemSizing::Lg, 1.5);
        sizing.insert(RemSizing::Xl, 2.0);

        Self {
            colours: Some(colours),
            sizing: Some(sizing),
        }
    }
}

#[derive(Default)]
pub struct Provider {
    theme: Mutable<Theme>,
    children: Vec<Dom>,
}

impl Provider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_theme(&mut self, theme: Theme) -> &mut Self {
        self.theme.set(Theme {
            colours: theme.colours.or(self.theme.get_cloned().colours),
            sizing: theme.sizing.or(self.theme.get_cloned().sizing),
        });
        self
    }

    pub fn child(&mut self, child: Dom) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn children(&mut self, children: Vec<Dom>) -> &mut Self {
        self.children.extend(children);
        self
    }

    // TODO : Return a fragment instead of a DOM
    pub fn dom(mut self) -> Dom {
        html!("div", { .child_signal(self.theme.clone().signal_cloned().map(move |theme| {

            THEME.set(theme.clone());
            Some(html!("div", {
                    .children(self.children.iter_mut().map(|c| c))
                }))

        })) })
    }
}
