/*
 Utilities for converting to and from various data types
*/

use url::Url;
use webdriver::Browser;

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
