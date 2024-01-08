use dominator::{class, html, pseudo, with_node};
use factoryizer::Factory;
use futures_signals::signal::{Mutable, SignalExt};
use tabler_dominator::icon;

use crate::{
    console_log,
    helpers::{
        colours::{bw_on_bg, opacity},
        mutable::Mutable2,
    },
};

use super::{
    ty::{Colour, Reactive},
    Component,
};

type SortFunction = fn(&str, &str) -> SortMovement;

#[derive(Clone, Default)]
pub enum TableDirection {
    #[default]
    Row,
    Column,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    #[default]
    None,
}

#[derive(Clone, Default)]
pub enum SortMovement {
    Up,
    Down,
    #[default]
    None,
}

#[derive(Clone, Default, Factory, Debug)]
pub struct TableValues {
    pub values: Vec<String>,
    pub title: String,
    pub sort: Option<SortFunction>,
}

impl TableValues {
    pub fn value(&mut self, value: &str) -> &mut Self {
        self.values.push(value.to_string());
        self
    }
}

pub fn numerical_sort(a: &str, b: &str) -> SortMovement {
    let a = a.parse::<f64>().unwrap_or(0.0);
    let b = b.parse::<f64>().unwrap_or(0.0);
    if a > b {
        SortMovement::Up
    } else if a < b {
        SortMovement::Down
    } else {
        SortMovement::None
    }
}

pub fn alphabetical_sort(a: &str, b: &str) -> SortMovement {
    if a > b {
        SortMovement::Up
    } else if a < b {
        SortMovement::Down
    } else {
        SortMovement::None
    }
}

#[derive(Factory, Clone, Default)]
pub struct Table {
    pub data: Mutable<Vec<TableValues>>,
    pub direction: TableDirection,
    colour: Colour,
    styles: Vec<(String, Reactive<String>)>,

    #[skip]
    applied_sort: Mutable<(String, SortDirection)>,
}

impl Table {
    pub fn sort_data(data: Vec<TableValues>, sort: (String, SortDirection)) -> Vec<TableValues> {
        let sort_values = data.iter().find(|container| container.title == sort.0);

        if sort_values.is_none() || sort.1 == SortDirection::None {
            return data;
        }

        let sort_values = sort_values.clone().unwrap();
        let mut sort_map = sort_values
            .values
            .iter()
            .enumerate()
            .map(|(i, value)| (i, value))
            .collect::<Vec<(usize, &String)>>();

        let sort_function = sort_values.sort.unwrap();
        sort_map.sort_by(|a, b| match sort_function(a.1, b.1) {
            SortMovement::Up => std::cmp::Ordering::Greater,
            SortMovement::Down => std::cmp::Ordering::Less,
            SortMovement::None => std::cmp::Ordering::Equal,
        });

        // Use indexes of sort map to sort data
        let mut sorted_data = Vec::new();
        for i in 0..data.len() {
            let mut container = TableValues::default();
            container.title = data[i].title.clone();
            container.sort = data[i].sort.clone();

            for (index, _) in sort_map.iter() {
                container.values.push(data[i].values[*index].clone());
            }

            if sort.1 == SortDirection::Descending {
                container.values.reverse();
            }

            sorted_data.push(container);
        }

        sorted_data
    }
}

impl Component for Table {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn dom(&mut self) -> dominator::Dom {
        html!("table", {
            .child_signal(Mutable2::new(self.data.clone(), self.applied_sort.clone()).map({
                let direction = self.direction.clone();
                let applied_sort = self.applied_sort.clone();
                let colour = self.colour.clone();
                move |(data, sort)| {

                    let data = Self::sort_data(data, sort.clone());

                    let titles = data.iter().map(|container| container.title.clone()).collect::<Vec<String>>();
                    let row_cells = match direction {
                        TableDirection::Row => {
                            data.iter().map(|container| container.values.clone()).collect::<Vec<Vec<String>>>()
                        },
                        TableDirection::Column => {
                            let mut cells = Vec::new();
                            for i in 0..titles.len() + 1 {
                                let mut column = Vec::new();
                                for container in data.iter() {
                                    column.push(container.values[i].clone());
                                }
                                cells.push(column);
                            }
                            cells
                        }
                    };

                    Some(html!("table", {
                        .class("w-full")
                        .class("table-auto")
                        .class("border-collapse")
                        .class("border")
                        .class("border-neutral-500")
                        .class("text-lg")
                        .class("overflow-clip")
                        .class("shadow-sm")
                        .class(
                            class! {
                                .pseudo!("> tr:nth-child(odd)", {
                                    .style("background", opacity(colour.to_string(), 0.15))
                                })
                            }
                        )
                        .child(html!("tr", {
                            .class(
                                class! {
                                    .style_important("background", opacity(colour.to_string(), 0.75))
                                    .style("color", bw_on_bg(opacity(colour.to_string(), 0.75)))
                                }
                            )
                            .children(data.iter().map(|container| {
                                html!("th", {
                                    .class("px-2")
                                    .class("border")
                                    .class("border-neutral-500")
                                    .child(html!("span", {
                                        .class("flex")
                                        .text(&container.title)
                                        .child(html!("button", {
                                            .class("ml-2")
                                            .with_node!(_e => {
                                                .event({
                                                    let sort = sort.clone();
                                                    let title = container.title.clone();
                                                    let applied_sort = applied_sort.clone();
                                                    move |_evt: dominator::events::Click| {
                                                        applied_sort.clone().set(
                                                            (
                                                                title.clone(),
                                                                match sort.1.clone() {
                                                                    SortDirection::Ascending => SortDirection::Descending,
                                                                    SortDirection::Descending => SortDirection::None,
                                                                    SortDirection::None => SortDirection::Ascending,
                                                                }
                                                            )
                                                        );
                                                    }
                                                })
                                            })
                                            .apply(|mut d| {
                                                if container.sort.is_some() {
                                                    if sort.0.clone() != container.title {
                                                        d = d.child(icon!("line-dashed"))
                                                    } else {
                                                        d = d.child(
                                                            match sort.1.clone() {
                                                                SortDirection::Ascending => icon!("chevron-up"),
                                                                SortDirection::Descending => icon!("chevron-down"),
                                                                SortDirection::None => icon!("line-dashed"),
                                                            }
                                                        );
                                                    }
                                                }

                                                d
                                            })
                                        }))
                                     }))
                                })
                            }))
                        }))
                        .children(row_cells.iter().map(|row| {
                            html!("tr", {
                                .class(
                                    class! {
                                        .pseudo!(":hover", {
                                            .style_important("background", opacity(colour.to_string(), 0.25))
                                        })
                                    }
                                )
                                .children(row.iter().map(|cell| {
                                    html!("td", {
                                        .text(cell)
                                        .class("px-2")
                                        .class("border")
                                        .class("border-neutral-500")
                                    })
                                }))
                            })
                        }))
                    }))

                }
            }))
        })
    }
}
