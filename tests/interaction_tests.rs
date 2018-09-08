extern crate selenium_rs;

use selenium_rs::webdriver::*;
use selenium_rs::element::Element;

fn get_search_bar(driver: &mut WebDriver) -> Element {
    driver.start_session();
    driver.navigate("http://google.com");
    driver.query_element(Selector::CSS, ".gLFyf").unwrap()
}

#[test]
fn test_enter_text() {
    let mut webdriver = WebDriver::new(Browser::Chrome);
    {
        let search_bar = get_search_bar(&mut webdriver);
        assert!(search_bar.type_text("testing").is_ok());
    }
    webdriver.delete_session();
}

#[test]
fn test_search() {
    let mut webdriver = WebDriver::new(Browser::Chrome);
    {
        get_search_bar(&mut webdriver).type_text("testing");
        let google_search_button = webdriver.query_element(Selector::CSS, "#gbqfbb").unwrap();
        assert!(google_search_button.click().is_ok());
    }
    // webdriver.delete_session();
}
