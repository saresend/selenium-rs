extern crate selenium_rs;

use selenium_rs::webdriver::*;
use selenium_rs::element::Element;

//TODO: Implement tests for interactability

fn get_search_bar(driver: &mut WebDriver) -> Element {
    driver.start_session();
    driver.navigate("http://google.com");
    driver.query_element(Selector::CSS, ".gLFyf").unwrap()
}

#[test]
fn test_enter_text() {
    let webdriver = WebDriver::new(Browser::Chrome).unwrap();
    let search_bar = get_search_bar();
    assert!(search_bar.type_text("testing").is_ok());
}

#[test]
fn test_search() {
    get_search_bar().type_text("testing");
}
