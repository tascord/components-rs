use dominator::{class, html};
use factoryizer::Factory;

use super::ty::{Component, Reactive, TextColour};

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

    styles: Vec<(String, Reactive<String>)>,
    id: Option<String>,
}

impl Component for Text {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn dom(&mut self) -> dominator::Dom {
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
            .class(
                class! {
                    .style("color", &self.colour.to_string())
                    .style(
                        "font-size",
                        &match self.variant {
                            TextVariant::Subscript => "0.83em",
                            TextVariant::Superscript => "0.83em",
                            TextVariant::Small => "0.75rem",
                            TextVariant::Default => "1rem",
                            TextVariant::H4 => "1.5rem",
                            TextVariant::H3 => "2rem",
                            TextVariant::H2 => "3rem",
                            TextVariant::H1 => "4.5rem",
                        }
                    )
                    .style(
                        "font-weight",
                        &match self.variant {
                            TextVariant::Subscript => "400",
                            TextVariant::Superscript => "400",
                            TextVariant::Small => "400",
                            TextVariant::Default => "400",
                            TextVariant::H4 => "600",
                            TextVariant::H3 => "700",
                            TextVariant::H2 => "700",
                            TextVariant::H1 => "700",
                        }
                    )
                }
            )
            .text(&self.text)
            .attr("id", &self.id.clone().unwrap_or_default())
            .apply(|mut d| {
                for (k, v) in self.styles.iter() {
                    d = v.apply_style(k.to_string(), d);
                }
                d
            })
        })
    }
}
