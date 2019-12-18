#![recursion_limit = "1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

pub mod agent;
pub mod components;
pub mod error;
pub mod routes;
pub mod types;

pub use components::app::App;
