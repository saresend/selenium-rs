use webdriver::*;
use session::session_structs::*;
use session::SessionHandler;


#[test]
fn test_new_session() {
    let mut webdriver = WebDriver::new(SupportedBrowser::Chrome);
    let new_session_params = NewSessionParams {
        desired_capabilities: DesiredCapabilities {
            browser_name: String::from("chrome"),
        }
    };
    let result = webdriver.new_session(new_session_params);
    assert!(result.is_ok());
}


#[test]
fn test_delete_session() {
    let mut webdriver = WebDriver::new(SupportedBrowser::Chrome);
    let new_session_params = NewSessionParams {
        desired_capabilities: DesiredCapabilities {
            browser_name: String::from("chrome"),
        }
    };
    let result = webdriver.new_session(new_session_params);
    assert!(result.is_ok());
    assert!(webdriver.delete_session().is_ok());
}

#[test]
fn test_get_status() {
     let mut webdriver = WebDriver::new(SupportedBrowser::Chrome);
    let new_session_params = NewSessionParams {
        desired_capabilities: DesiredCapabilities {
            browser_name: String::from("chrome"),
        }
    };
    let result = webdriver.new_session(new_session_params);
    assert!(result.is_ok());
    println!("{:?}", webdriver.status());
    assert!(webdriver.status().is_ok());
    assert!(webdriver.delete_session().is_ok())
}

