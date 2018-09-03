extern crate selenium_rs;

use selenium_rs::webdriver::{Browser, Selector, WebDriver};

#[test]
fn test_intialization() {
    let mut driver = WebDriver::new(Browser::Chrome);
    assert!(driver.start_session().is_ok());
}

#[test]
fn test_navigation() {
    let mut driver = WebDriver::new(Browser::Chrome);
    assert!(driver.start_session().is_ok());
    assert!(driver.navigate("http://google.com").is_ok());
}

#[test]
fn test_element_search_by_id() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("http://google.com");
    driver.query_element(Selector::CSS, "#searchform").is_ok();
}
