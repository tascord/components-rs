use dominator::{class, html, with_node, pseudo};
use factoryizer::Factory;

use super::ty::{Component, Reactive};
use lazy_static::lazy_static;

lazy_static! {
    static ref LOOP: String =
        String::from_utf8_lossy(include_bytes!("../../assets/logo/Loop.svg")).to_string();
    static ref TEXT: String =
        String::from_utf8_lossy(include_bytes!("../../assets/logo/Text.svg")).to_string();
    static ref COMBINATION: String = format!("{}{}", &*LOOP, &*TEXT);
}

#[derive(Default, Clone)]
pub enum BrandVariant {
    Mark,
    Text,
    #[default]
    MarkText,
}

#[derive(Default, Clone)]
pub enum BrandColour {
    #[default]
    Light,
    Dark,
}

#[derive(Factory, Default)]
pub struct Brand {
    variant: BrandVariant,
    colour: BrandColour,

    styles: Vec<(String, Reactive<String>)>,
}

impl Component for Brand {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn dom(&mut self) -> dominator::Dom {
        html!("div" => web_sys::HtmlElement, {
            .class(
                class! {
                    .style("display", "flex")
                    .style("flex-wrap", "nowrap")
                    .style("justify-content", "center")
                    .style("align-items", "center")
                    .style("height", "100%")
                    .style(
                        "fill",
                         match self.colour {
                            BrandColour::Light => "#fff",
                            BrandColour::Dark => "#000",
                        }
                    )
                    .pseudo!(" svg", {
                        .style("height", "100%")
                        .style("margin", "auto")
                    })
                    .pseudo!("> * + *", {
                        .style("margin-left", "0.5rem")
                    })
                }
            )
            .apply(|mut d| {
                for (k, v) in self.styles.iter() {
                    d = v.apply(k.to_string(), d)
                }
                d
            })
            .with_node!(element => {
                .text({
                    element.set_inner_html({
                        match self.variant {
                            BrandVariant::Mark => &*LOOP,
                            BrandVariant::Text => &*TEXT,
                            BrandVariant::MarkText => &*COMBINATION,
                        }
                    });
                    ""
                })
            })
        })
    }

}
