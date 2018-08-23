use webdriver::SupportedBrowser;

pub fn get_string_rep(browser: SupportedBrowser) -> String {
    match browser {
        SupportedBrowser::Chrome => String::from("chrome")
    }
}