use dominator::{html, Dom, DomBuilder};
use factoryizer::Factory;
use web_sys::HtmlElement;

use crate::helpers::css::{State, CSS};

use super::ty::{Component, RemSizing};

#[derive(Default, Clone)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl ToString for FlexDirection {
    fn to_string(&self) -> String {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::Column => "column",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::ColumnReverse => "column-reverse",
        }
        .to_string()
    }
}

#[derive(Factory, Default)]
pub struct Flex {
    children: Vec<DomBuilder<HtmlElement>>,
    space_x: RemSizing,
    space_y: RemSizing,
    wrap: bool,
    direction: FlexDirection,
}

impl Flex {
    pub fn child(&mut self, child: DomBuilder<HtmlElement>) -> &mut Self {
        self.children.push(child);
        self
    }
}

impl Component for Flex {
    fn render(&mut self, id: String) -> dominator::Dom {
        html!("div", {
            .attr("id", &id)
            .children(self.children.into_iter().map(|d| d.into_dom()).collect::<Vec<Dom>>())
        })
    }
    fn css(&self) -> CSS {
        let c = CSS::new()
            .add_state(
                None,
                State::new()
                    .add_property("display", "flex")
                    .add_property("flex-direction", &self.direction.to_string())
                    .add_property(
                        "flex-wrap",
                        &match self.wrap {
                            true => "wrap",
                            false => "nowrap",
                        },
                    )
                    .clone(),
            )
            .add_state(
                Some("> * + *"),
                State::new()
                    .add_property(
                        "margin",
                        &format!("{} {}", self.space_y.to_string(), self.space_x.to_string()),
                    )
                    .clone(),
            )
            .clone();
        c
    }
}
