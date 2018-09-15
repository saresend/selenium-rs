extern crate selenium_rs;
use selenium_rs::element::Element;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};

#[test]
fn test_delete_without_session() {
    let mut driver = WebDriver::new(Browser::Chrome);
}
