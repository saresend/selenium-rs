extern crate selenium_rs;

use selenium_rs::webdriver::{WebDriver, Browser};

#[test] 
fn test_something() {
  let driver = WebDriver::new(Browser::Chrome);
  assert!(driver.start_session().is_ok());
}