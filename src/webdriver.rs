use reqwest;

pub enum Browser {
  Chrome, 
  Firefox,
}
pub struct WebDriver {
  browser: Browser,
}

impl WebDriver {

  pub fn new(browser: Browser) -> WebDriver {
    WebDriver { browser } 
  }

  pub fn create_session() -> reqwest::Result<()> {
    
  }
}