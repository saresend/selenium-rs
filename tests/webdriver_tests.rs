extern crate selenium_rs;

use selenium_rs::webdriver::{Browser, WebDriver};

#[test]
fn test_intialization() {
    let mut driver = WebDriver::new(Browser::Chrome);
    assert!(driver.start_session().is_ok());
}

#[test]
fn test_navigation() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    assert!(driver.navigate("http://google.com").is_ok());
}
