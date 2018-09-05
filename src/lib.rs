extern crate reqwest;
extern crate url;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod element;
pub mod webdriver;

mod element_structs;
mod session_structs;
mod utils;
