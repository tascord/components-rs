
use dominator::{html, Dom};
use factoryizer::Factory;

use crate::helpers::css::{State, CSS};

use super::ty::{Component, RemSizing, Reactive};

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
    as_tag: Option<&'static str>,
    space_x: RemSizing,
    space_y: RemSizing,
    wrap: bool,
    direction: FlexDirection,
    
    #[skip]
    children: Vec<Dom>,
    #[skip]
    styles: Vec<(String, Reactive<String>)>,
}

impl Flex {
    pub fn child(&mut self, child: Dom) -> &mut Self {
        self.children.push(child);
        self
    }
}

impl Component for Flex {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn render(&mut self, class: String) -> dominator::Dom {
        html!(self.as_tag.unwrap_or("div"), {
            .class(&class)
            .children(self.children.iter_mut().map(|c| c))
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
                    .add_property("margin-left", &self.space_x.to_string())
                    .add_property("margin-top", &self.space_y.to_string())
                    .clone(),
            )
            .clone();
        c
    }
}
