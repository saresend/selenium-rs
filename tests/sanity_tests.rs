extern crate selenium_rs;
use selenium_rs::webdriver::{Browser, WebDriver};

#[test]
fn test_delete_without_session() {
    let _driver = WebDriver::new(Browser::Chrome);
}
