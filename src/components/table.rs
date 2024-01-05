use dominator::{html, with_node};
use factoryizer::Factory;
use futures_signals::signal::{Mutable, SignalExt};
use tabler_dominator::icon;

use crate::{
    console_log,
    helpers::{css::CSS, mutable::Mutable2},
};

use super::{ty::Reactive, Component};

type SortFunction = fn(&str, &str) -> SortMovement;

#[derive(Clone, Default)]
pub enum TableDirection {
    #[default]
    Row,
    Column,
}

#[derive(Debug, Clone, Default)]
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
    styles: Vec<(String, Reactive<String>)>,
    applied_sort: Mutable<(String, SortDirection)>,
}

impl Table {
    pub fn sort_data(data: Vec<TableValues>, sort: (String, SortDirection)) -> Vec<TableValues> {
        let sort_values = data.iter().find(|container| container.title == sort.0);

        if sort_values.is_none() {
            return data;
        }

        let sort_values = sort_values.clone().unwrap();
        let mut sort_map = sort_values
            .values
            .iter()
            .enumerate()
            .map(|(i, value)| (i, value))
            .collect::<Vec<(usize, &String)>>();

        let sort_function = alphabetical_sort;  // sort_values.sort.clone().unwrap_or(alphabetical_sort);

        sort_map.sort_by(|a, b| {
            match sort_function(a.1, b.1) {
                SortMovement::Up => std::cmp::Ordering::Greater,
                SortMovement::Down => std::cmp::Ordering::Less,
                SortMovement::None => std::cmp::Ordering::Equal,
            }
        });

        // Use indexes of sort map to sort data
        let mut sorted_data = Vec::new();
        for i in 0..data.len() {
            let mut container = TableValues::default();
            container.title = data[i].title.clone();
            for (index, _) in sort_map.iter() {
                container.values.push(data[i].values[*index].clone());
            }
            sorted_data.push(container);
        }

        console_log!("Sorted", sorted_data.clone());
        sorted_data
    }
}

impl Component for Table {
    fn style(&mut self, style: (String, Reactive<String>)) -> &mut Self {
        self.styles.push(style);
        self
    }
    fn render(&mut self, _: String) -> dominator::Dom {
        html!("table", {
            .child_signal(Mutable2::new(self.data.clone(), self.applied_sort.clone()).map({
                let direction = self.direction.clone();
                let applied_sort = self.applied_sort.clone();
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
                        .child(html!("tr", {
                            .class("flex")
                            .class("flex-nowrap")
                            .children(data.iter().map(|container| {
                                html!("th", {
                                    .class("flex")
                                    .class("items-center")
                                    .class("justify-center")
                                    .class("space-x-2")
                                    .class("flex-nowrap")
                                    .child(html!("span", { .text(&container.title) }))
                                    .child(html!("button", {
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
                                            console_log!("Container", container);
                                            if container.sort.is_some() {
                                                if sort.0.clone() != container.title {
                                                    d = d.child(icon!("point-filled"))
                                                } else {
                                                    d = d.child(
                                                        match sort.1.clone() {
                                                            SortDirection::Ascending => icon!("chevron-up"),
                                                            SortDirection::Descending => icon!("chevron-down"),
                                                            SortDirection::None => icon!("point-filled"),
                                                        }
                                                    );
                                                }
                                            }

                                            d
                                        })
                                    }))
                                })
                            }))
                        }))
                        .children(row_cells.iter().map(|row| {
                            html!("tr", {
                                .children(row.iter().map(|cell| {
                                    html!("td", {
                                        .text(cell)
                                    })
                                }))
                            })
                        }))
                    }))

                }
            }))
        })
    }

    fn css(&self) -> crate::helpers::css::CSS {
        CSS::new()
    }
}
