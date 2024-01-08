use std::rc::Rc;

use dominator::{class, html, pseudo, with_node, Dom};
use factoryizer::Factory;

use crate::helpers::colours::{bw_on_bg, darken, opacity, TRANSPARENT};

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
    value: Reactive<String>,
    variant: ButtonVariant,
    colour: Colour,
    size: RemSizing,
    radius: RemSizing,
    padding: RemSizing,

    #[skip]
    on_click: Option<Rc<dyn Fn()>>,
    #[skip]
    styles: Vec<(String, Reactive<String>)>,
    #[skip]
    classes: Vec<String>,
    #[skip]
    children: Vec<Dom>,
}

impl Button {
    pub fn text(&mut self, text: &str) -> &mut Self {
        self.value = Reactive::Static(text.to_string());
        self
    }
    pub fn on_click(&mut self, closure: impl Fn() + 'static) -> &mut Self {
        self.on_click = Some(Rc::new(closure));
        self
    }
    pub fn child(&mut self, child: Dom) -> &mut Self {
        self.children.push(child);
        self
    }
}

impl Component for Button {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }

    fn dom(&mut self) -> dominator::Dom {
        html!("button", {
            .with_node!(_e => {
                .event({
                    let on_click = self.on_click.clone().unwrap_or(Rc::new(|| {}));
                    move |_evt: dominator::events::Click| {
                        on_click();
                    }
                })
                .class("font-semibold")
                .class(
                    class! {
                        .style("padding", &format!("{} {}", self.padding.mult(0.5).to_string(), self.padding.to_string()))
                        .style("border-radius", &self.radius.mult(0.45).to_string())
                        .style("font-size", &self.size.to_string())
                        .style("color", match self.variant {
                            ButtonVariant::Filled => bw_on_bg(self.colour.to_string()),
                            _ => self.colour.to_string(),
                        })
                        .style("outline", &match self.variant {
                            ButtonVariant::Outline => format!("2px solid {}", self.colour.to_string()),
                            _ => TRANSPARENT.to_string(),
                        })
                        .style("background", &match self.variant {
                            ButtonVariant::Filled => self.colour.to_string(),
                            ButtonVariant::Light => opacity(self.colour.to_string(), 0.15),
                            ButtonVariant::Subtle | ButtonVariant::Outline => {
                                TRANSPARENT.to_string()
                            }
                        })
                        .pseudo!(":hover", {
                            .style("background", &match self.variant {
                                ButtonVariant::Filled => darken(self.colour.to_string(), 0.25),
                                ButtonVariant::Light => opacity(self.colour.to_string(), 0.25),
                                ButtonVariant::Subtle | ButtonVariant::Outline => opacity(self.colour.to_string(), 0.15),
                            })
                        })
                        .pseudo!(":active", {
                            .style("transform", "translateY(0.15rem)")
                        })
                    }
                )
            })
            .children(self.children.iter_mut().map(|c| c))
            .apply(|mut d| {
                d = self.value.apply_text(d);
                for (k, v) in self.styles.iter() {
                    d = v.apply_style(k.to_string(), d);
                }
                for c in self.classes.iter() {
                    d = d.class(c);
                }
                d
            })
        })
    }
}
