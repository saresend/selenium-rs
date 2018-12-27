extern crate selenium_rs;
use selenium_rs::element::Element;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};

fn get_element(driver: &mut WebDriver) -> Element {
    driver.start_session();
    driver.navigate("http://google.com");
    driver.query_element(Selector::CSS, "#searchform").unwrap()
}

#[test]
fn test_get_multiple_elements() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("http://google.com");
    let elements = driver.query_element(Selector::CSS, "a").unwrap();
    println!("{:?}", elements);
}

#[test]
fn test_is_selected() {
    let mut driver = WebDriver::new(Browser::Chrome);
    let search_form = get_element(&mut driver);
    assert_eq!(search_form.is_selected().unwrap(), false);
}

#[test]
fn test_get_attribute() {
    let mut driver = WebDriver::new(Browser::Chrome);
    let search_form = get_element(&mut driver);
    assert_eq!(
        search_form.get_attribute("class").unwrap(),
        String::from("jhp")
    );
}

#[test]
fn test_get_property() {
    let mut driver = WebDriver::new(Browser::Chrome);
    let search_form = get_element(&mut driver);
    // Isn't supported by the webdriver I'm testing, apparently...
}

#[test]
fn test_css_value() {
    let mut driver = WebDriver::new(Browser::Chrome);
    let search_form = get_element(&mut driver);
    assert_eq!(
        search_form.get_css_value("min-width").unwrap(),
        String::from("980px")
    );
}

#[test]
fn test_get_text() {
    let mut driver = WebDriver::new(Browser::Chrome);
    let search_form = get_element(&mut driver);
    assert!(search_form.get_text().unwrap().contains("Gmail\n"));
}
