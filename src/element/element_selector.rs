
use element::Element;
use webdriver::WebDriver;

pub enum SelectStrategy {
  CSS,
  LinkText,
  PartialLinkText,
  TagName,
  XPath, 
}

fn get_string_for_strategy(strat: &SelectStrategy) -> String {
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

struct ElementRequestPayload<'a, 'b> {
  using: &'a str, 
  value: &'b str,
}

impl<'a, 'b> ElementRequestPayload<'a, 'b> {
  pub fn new(using: &'a str, value: &'b str) -> ElementRequestPayload<'a, 'b> {
    ElementRequestPayload { using, value }
  }
}

impl<'a> ElementRequest<'a> {
  pub fn new(strat: SelectStrategy, params: &'a str) -> ElementRequest<'a> {
    ElementRequest {
      strategy: strat, 
      params: params,
    }
  }

  pub fn get_element(&self, webdriver: &WebDriver) -> Element {
    // Make a request to the correct url and get the element ID 
    let string_for_strategy = get_string_for_strategy(&self.strategy);
    let payload = ElementRequestPayload::new(&string_for_strategy, self.params);
    unimplemented!();
  }
}



