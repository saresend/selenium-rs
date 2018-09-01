use reqwest;
use session_structs::{NewSessionRequest, NewSessionResponse};
use utils::{construct_url, get_browser_string};

pub enum Browser {
    Chrome,
    Firefox,
}
pub struct WebDriver {
    browser: String,
    client: reqwest::Client,
    session_id: Option<String>,
}

// Contains Creation Methods
impl WebDriver {
    pub fn new(browser: Browser) -> WebDriver {
        let browser = get_browser_string(browser);
        WebDriver {
            browser,
            client: reqwest::Client::new(),
            session_id: None,
        }
    }
}

// Contains Session Handling
impl WebDriver {
    pub fn start_session(&mut self) -> reqwest::Result<()> {
        let body = NewSessionRequest::new(&self.browser);
        let url = construct_url(vec!["session/"]);

        let response: NewSessionResponse = self.client
            .post(url)
            .json(&body)
            .send()?
            .error_for_status()?
            .json()?;

        self.session_id = Some(response.get_session_id());
        Ok(())
    }
}
