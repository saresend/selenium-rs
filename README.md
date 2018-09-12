# Selenium-rs
[![GitHub issues](https://img.shields.io/github/issues/saresend/selenium-rs.svg)](https://github.com/saresend/selenium-rs/issues)

## About 

Selenium-rs is a simple client for the [selenium webdriver](https://www.seleniumhq.org/). Its built to work with the webdriver protocol (spec found [here](https://www.w3.org/TR/webdriver1/)). It currently supports the chrome Driver, and Gecko (firefox) support is on the way. 

## Installation 

```toml
[dependencies]
selenium-rs = "0.1.0"

```

Note that selenium-rs also requires a running instance of the selenium webdriver, which can be found [here](https://www.seleniumhq.org/download/). Simply download the jar and run it to instantiate the selenium webdriver server. 

## Documentation: [docs.rs](https://docs.rs/selenium-rs/0.1.0/selenium_rs/)

## Sample Usage 

### Example - Navigating to a web page 
```rust 
use selenium_rs::webdriver::{Browser,WebDriver};

let mut driver= WebDriver::new(Browser::Chrome);
driver.start_session();

driver.navigate("https://www.rust-lang.org"); 
assert_eq!(driver.get_current_url().unwrap(), String::from("https://www.rust-lang.org/en-US/"));
```

### Performing a google search 
```rust 
use selenium_rs::webdriver::{Browser, WebDriver, Selector};
let mut driver = WebDriver::new(Browser::Chrome);

driver.start_session();
driver.navigate("http://google.com");
let search_bar = driver.query_element(Selector::CSS, ".gLFyf").unwrap();

search_bar.type_text("selenium-rs github");
let search_button = driver.query_element(Selector::CSS, "#gbqfbb").unwrap();
search_button.click();
```

### Example - Inspecting attributes of an element 
```rust 
    use selenium_rs::webdriver::{Selector, Browser, WebDriver};
    use selenium_rs::element::Element;
    
    let mut driver = WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("http://www.google.com");
    let search_form =  driver.query_element(Selector::CSS, "#searchform").unwrap();
    assert!(search_form.get_css_value("min-width").unwrap() == "980px");
```

## Current Status 

Currently, the project supports many of the more important functionalities provided by the webdriver spec. However, it is still
a fair bit away from complete. Support will get periodically added as I find the time to finish implementing everything described in the webdriver spec. In-progress features will be tracked [here](https://github.com/saresend/selenium-rs/issues). 

**Note:** Currently only tested with Selenium 3.14


## Contribution 

Pull requests are always welcome! Please see the [issue tracker](https://github.com/saresend/selenium-rs/issues) for currently in-progress features, improvements, and bugfixes.


## Licence 
This is provided under the [MIT](https://github.com/saresend/selenium-rs/blob/master/LICENSE) license. 
