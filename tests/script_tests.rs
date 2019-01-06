extern crate selenium_rs;
use selenium_rs::element::Element;
use selenium_rs::webdriver::{Browser, Selector, WebDriver};

#[test]
fn test_execute() {
	let mut driver = WebDriver::new(Browser::Chrome);
	driver.start_session();
	driver.navigate("http://google.com");
	let answer: i32 = driver.execute_script("return 2+2;", &[]).unwrap();
	assert_eq!(answer, 4);
}

#[test]
fn test_arguments() {
	let mut driver = WebDriver::new(Browser::Chrome);
	driver.start_session();
	driver.navigate("http://google.com");
	let answer: serde_json::Value = driver.execute_script("return arguments[0]+arguments[1];", &[2.into(), Vec::<i32>::new().into()]).unwrap();
	assert_eq!(answer.to_string(), "\"2\"");
}