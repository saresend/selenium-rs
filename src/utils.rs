/*
 Utilities for converting to and from various data types
*/

use url::Url;
use webdriver::{Browser, Selector};

pub fn get_browser_string(browser: Browser) -> String {
    match browser {
        Browser::Chrome => String::from("chrome"),
        Browser::Firefox => String::from("firefox"),
    }
}

pub fn get_default_url() -> Url {
    // Safe unwrap here, since its a hardcoded, valid url
    Url::parse("http://localhost:4444/wd/hub/").unwrap()
}

pub fn construct_url(elements: Vec<&str>) -> Url {
    let mut url = get_default_url();
    for element in elements {
        url = url.join(element).unwrap();
    }
    url
}

pub fn str_for_selector(selector: Selector) -> String {
    match selector {
        Selector::CSS => String::from("css selector"),
        Selector::LinkText => String::from("link text"),
        Selector::PartialLinkText => String::from("partial link text"),
        Selector::TagName => String::from("tag name"),
        Selector::XPath => String::from("xpath"),
    }
}
