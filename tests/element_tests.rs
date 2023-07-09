extern crate selenium_rs;
use selenium_rs::element::Element;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};

fn get_element(driver: &mut WebDriver) -> Element {
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    driver.find_element(Selector::CSS, "#searchform").unwrap()
}

#[test]
fn test_get_multiple_elements_find() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    let elements = driver.find_element(Selector::CSS, "a").unwrap();
    println!("{:?}", elements);
}

#[test]
fn test_find_single_element_under_element() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    let parent_element = driver.find_element(Selector::CSS, "html").unwrap();

    let meta_elements_under_parent = parent_element.find_element(Selector::CSS, ".csi").unwrap();
    println!("{:?}", meta_elements_under_parent);
}

#[test]
fn test_find_multiple_elements_under_element() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    let parent_element = driver.find_element(Selector::CSS, "html").unwrap();

    let meta_elements_under_parent = parent_element.find_elements(Selector::CSS, "meta").unwrap();
    println!("{:?}", meta_elements_under_parent);
    assert!(!meta_elements_under_parent.is_empty())
}

#[test]
fn test_get_multiple_elements_query() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    let elements = driver.find_element(Selector::CSS, "a").unwrap();
    println!("{:?}", elements);
}

#[test]
fn test_get_element_list_find() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    assert_eq!(
        driver
            .find_elements(Selector::CSS, "#searchform")
            .unwrap()
            .len(),
        1
    );
    assert!(driver.find_elements(Selector::CSS, "a").unwrap().len() > 1);
}

#[test]
fn test_get_element_list_query() {
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session().unwrap();
    driver.navigate("http://google.com").unwrap();
    assert_eq!(
        driver
            .find_elements(Selector::CSS, "#searchform")
            .unwrap()
            .len(),
        1
    );
    assert!(driver.find_elements(Selector::CSS, "a").unwrap().len() > 1);
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
    let _search_form = get_element(&mut driver);
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
