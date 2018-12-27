/*! 
    
    This provides the primary point of interaction with the Selenium WebDriver API. We 
    can use it to create and manage sessions, as well as use it to spawn elements from the 
    current browsing context. 

    # Example - Navigating to a web page 

    ```rust 
    use selenium_rs::webdriver::{Browser,WebDriver};
    
    let mut driver= WebDriver::new(Browser::Chrome);
    driver.start_session();
    driver.navigate("https://www.rust-lang.org"); 
    assert_eq!(driver.get_current_url().unwrap(), String::from("https://www.rust-lang.org/"));
    ```
*/

use element::Element;
use element_structs::{ElementResponse, ElementsResponse, ExecuteScriptResponse};
use reqwest;
use session_structs::{NewSessionRequest, NewSessionResponse, TitleResponse};
use std::collections::HashMap;
use utils::*;

pub enum Browser {
    Chrome,
    Firefox,
}

pub enum Selector {
    CSS,
    LinkText,
    PartialLinkText,
    TagName,
    XPath,
}

#[derive(Serialize, Deserialize)]
struct ElementRequest {
    using: String,
    value: String,
}

impl ElementRequest {
    pub fn new(using: String, value: String) -> ElementRequest {
        ElementRequest { using, value }
    }
}

#[derive(Serialize)]
struct ExecuteScriptRequest {
    script: String,
    args: Vec<serde_json::Value>,
}

impl ExecuteScriptRequest {
    pub fn new(script: String, args: Vec<serde_json::Value>) -> ExecuteScriptRequest {
        ExecuteScriptRequest { script, args }
    }
}

/// The WebDriver is the primary way by which interaction
/// is managed between the Selenium-Webdriver Server, and
/// our client.
pub struct WebDriver {
    browser: String,
    client: reqwest::Client,
    session_id: Option<String>,
}

// Contains Creation Methods
impl WebDriver {
    /// Constructs a new Webdriver with the specific browser.
    /// TODO: Make sure and add testing to verify that it supports
    /// firefox properly
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
    /// Actually starts and creates a session on the server,
    /// collecting the session ID on success, and returning an error
    /// on failure
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
    /// Returns the current url of the browsing context. See examples for
    /// more details on how this is used
    pub fn get_current_url(&self) -> reqwest::Result<String> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone().unwrap() + "/"),
            "url",
        ]);
        let response: TitleResponse = self.client.get(url).send()?.error_for_status()?.json()?;

        Ok(response.get_title())
    }

    pub fn get_title(&self) -> reqwest::Result<String> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone().unwrap() + "/"),
            "title",
        ]);

        let response: TitleResponse = self.client.get(url).send()?.error_for_status()?.json()?;

        Ok(response.get_title())
    }
}

// Contains Navigation Handling
impl WebDriver {
    /// Navigates the current browsing session to the reqwested url. This will
    /// also wait until the browser has finished executing the request, meaning that
    /// future calls may assume we've reached the appropriate url
    pub fn navigate(&self, url: &str) -> reqwest::Result<()> {
        let sess_id = self.session_id.clone().unwrap();
        let nav_url = construct_url(vec!["session/", &(sess_id + "/"), "url"]);
        let mut payload = HashMap::new();
        payload.insert("url", url);
        self.client
            .post(nav_url)
            .json(&payload)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    pub fn back(&self) -> reqwest::Result<()> {
        let sess_id = self.session_id.clone().unwrap();
        let nav_url = construct_url(vec!["session/", &(sess_id + "/"), "back"]);
        self.client.post(nav_url).send()?.error_for_status()?;
        Ok(())
    }
}

// Contains Element Handling
impl WebDriver {
    /// Requests an elements from the webpage, given the specified selector and query string
    pub fn query_element(&self, selector: Selector, query: &str) -> reqwest::Result<Element> {
        let sess_id = self.session_id.clone().unwrap();
        let url = construct_url(vec!["session/", &(sess_id + "/"), "element"]);
        let payload = ElementRequest::new(str_for_selector(selector), query.to_string());
        let response: ElementResponse = self.client
            .post(url)
            .json(&payload)
            .send()?
            .error_for_status()?
            .json()?;
        let element = response.parse_into_element(&self.client);
        Ok(element)
    }

    /// Requests a list of elements from the webpage, given the specified selector and query string
    pub fn query_elements(&self, selector: Selector, query: &str) -> reqwest::Result<Vec<Element>> {
        let sess_id = self.session_id.clone().unwrap();
        let url = construct_url(vec!["session/", &(sess_id + "/"), "elements"]);
        let payload = ElementRequest::new(str_for_selector(selector), query.to_string());
        let response: ElementsResponse = self.client
            .post(url)
            .json(&payload)
            .send()?
            .error_for_status()?
            .json()?;
        let elements = response.parse_into_elements(&self.client);
        Ok(elements)
    }
}

// Contains Document Handling
impl WebDriver {
    /// Executes the given script synchronously and returns the result
    pub fn execute_script<T: serde::de::DeserializeOwned>(&self, script: &str, args: &[serde_json::Value]) -> reqwest::Result<T> {
        let sess_id = self.session_id.clone().unwrap();
        let url = construct_url(vec!["session/", &(sess_id + "/"), "execute/sync"]);
        let payload = ExecuteScriptRequest::new(script.to_string(), args.to_owned());
        let response: ExecuteScriptResponse<T> = self.client
            .post(url)
            .json(&payload)
            .send()?
            .error_for_status()?
            .json()?;
        Ok(response.value)
    }
}

impl Drop for WebDriver {
    fn drop(&mut self) {
        if let Some(ref id) = self.session_id {
            let url = construct_url(vec!["session/", id]);
            let _ = self.client.delete(url).send();
        }
    }
}
