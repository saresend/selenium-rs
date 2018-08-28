
use element::Element;

pub enum SelectStrategy {
  CSS,
  LinkText,
  PartialLinkText,
  TagName,
  XPath, 
}

fn get_string_for_strategy(strat: SelectStrategy) -> String {
  match strat {
    SelectStrategy::CSS => String::from("css selector"),
    SelectStrategy::LinkText => String::from("link text"),
    SelectStrategy::PartialLinkText => String::from("partial link text"),
    SelectStrategy::TagName => String::from("tag name"),
    SelectStrategy::XPath => String::from("xpath"),
  }
}

pub struct ElementRequest<'a> {
  strategy: SelectStrategy, 
  params: &'a str, 
}

impl<'a> ElementRequest<'a> {
  pub fn new(strat: SelectStrategy, params: &'a str) -> ElementRequest<'a> {
    ElementRequest {
      strategy: strat, 
      params: params,
    }
  }

  pub fn get_element(&self) -> Element {
    unimplemented!();
  }
}

