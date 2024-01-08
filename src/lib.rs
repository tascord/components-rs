pub mod components;
pub mod helpers;

pub static TAILWIND: &'static [u8] = include_bytes!("../assets/tw.css"); 

#[cfg(feature = "example")]
pub mod example;
