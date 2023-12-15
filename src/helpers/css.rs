use std::collections::HashMap;

use dominator::{html, Dom};
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GENERATED_CSS: Mutable<HashMap<String, String>> = Mutable::new(HashMap::new());
    pub static ref GENERIC_CSS: Mutable<Vec<String>> = Mutable::new(vec![
        String::from("body { margin: 0; padding: 0; font-family: 'Montserrat', sans-serif; }"),
    ]);
}

pub fn style_element() -> Dom {
    html!("style", {
        .text("/* Generic CSS */")
        .text_signal(GENERIC_CSS.signal_cloned().map(|css| css.join("\n")))
        .text("\n\n/* Generated CSS */")
        .text_signal(GENERATED_CSS.signal_cloned().map(|css| {
            css.into_iter().map(|(id, css)| format!("#{id} {{ {css} }}")).collect::<Vec<String>>().join("\n")
        }))
    })
}

pub fn add_css(id: String, css: String) {
    GENERATED_CSS.set(
        GENERATED_CSS
            .get_cloned()
            .into_iter()
            .chain(vec![(id, css)])
            .collect(),
    )
}

#[derive(Clone)]
pub struct State {
    pub properties: HashMap<String, String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, key: &str, value: &str) -> &mut Self {
        self.properties.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Clone)]
pub struct CSS {
    identifier: String,
    pub states: HashMap<String, State>,
}

impl CSS {
    pub fn new() -> Self {
        Self {
            identifier: CSS::generate_identifier(),
            states: HashMap::new(),
        }
    }

    pub fn add_state(&mut self, state: Option<&str>, properties: State) -> &mut Self {
        self.states.insert(state.unwrap_or_default().to_string(), properties);
        self
    }

    pub fn generate(self) -> String {
        self.states.iter().for_each(|(state, properties)| {
            let mut css = String::new();
            properties.properties.iter().for_each(|(key, value)| {
                css.push_str(&format!("{}: {};\n", key, value));
            });
            add_css(format!("{}{}", self.identifier, {
                if state.is_empty() {
                    "".to_string()
                } else {
                    format!(":{}", state)
                }
            }), css);
        });

        self.identifier
    }

    fn generate_identifier() -> String {
        let mut identifier = String::new();
        for _ in 0..8 {
            let random = (rand::random::<f32>() * 26.0) as u8 + 97;
            identifier.push(random as char);
        }
        identifier
    }
}
