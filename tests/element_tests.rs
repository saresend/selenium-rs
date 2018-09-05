extern crate selenium_rs;
use selenium_rs::element::Element;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};

fn get_element(driver: &mut WebDriver) -> Element {
  driver.start_session();
  driver.navigate("http://google.com");
  driver.query_element(Selector::CSS, "#searchform").unwrap()
}


#[test] 
fn test_is_selected() {
  let mut driver = WebDriver::new(Browser::Chrome);
  let search_form = get_element(&mut driver);
  assert!(search_form.is_selected().unwrap() == false);
}