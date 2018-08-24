extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod macros;
mod selenium_core;

pub mod session;
pub mod navigation;

pub mod webdriver;

extern crate url;
