# Selenium-rs
[![GitHub issues](https://img.shields.io/github/issues/saresend/selenium-rs.svg)](https://github.com/saresend/selenium-rs/issues)

## About 

Selenium-rs is a simple client for the [selenium webdriver](https://www.seleniumhq.org/). Its built to work with the webdriver protocol (spec found [here](https://www.w3.org/TR/webdriver1/)). It currently supports the chrome Driver, and Gecko (firefox) support is on the way. 

## Installation 

```toml
[dependencies]
selenium-rs = "0.1.0"

```

## Documentation: [docs.rs](https://docs.rs/selenium-rs/0.1.0/selenium_rs/)

## Current Status 

Currently, the project supports many of the more important functionalities provided by the webdriver spec. However, it is still
a fair bit away from complete. Support will get periodically added as I find the time to finish implementing everything described in the webdriver spec. In-progress features will be tracked [here](https://github.com/saresend/selenium-rs/issues). 

**Note:** Currently only tested with Selenium 3.14


## Contribution 

Pull requests are always welcome! Please see the [issue tracker](https://github.com/saresend/selenium-rs/issues) for currently in-progress features, improvements, and bugfixes.


## Licence 
This is provided under the [MIT](https://github.com/saresend/selenium-rs/blob/master/LICENSE) license. 
