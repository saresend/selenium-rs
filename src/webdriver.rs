use reqwest;
use session_structs::NewSessionRequest;
use utils::get_browser_string;

pub enum Browser {
    Chrome,
    Firefox,
}
pub struct WebDriver {
    browser: String,
    client: reqwest::Client,
}

// Contains Creation Methods
impl WebDriver {
    pub fn new(browser: Browser) -> WebDriver {
        let browser = get_browser_string(browser);
        WebDriver {
            browser,
            client: reqwest::Client::new(),
        }
    }
}

// Contains Session Handling
impl WebDriver {
    pub fn start_session(&self) -> reqwest::Result<()> {
        let body = NewSessionRequest::new(&self.browser);
        unimplemented!();
    }
}
