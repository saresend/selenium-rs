extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod macros;
mod selenium_core;

mod session;

pub mod webdriver;
mod webdriver_tests;

extern crate url;
