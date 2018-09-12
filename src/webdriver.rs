use element::{Element, ElementRequest};
use element_structs::ElementResponse;
use reqwest;
use session_structs::{NewSessionRequest, NewSessionResponse, TitleResponse};
use std::collections::HashMap;
use utils::*;

use serde_json;

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

    /// Deletes the session, consuming itself in the process
    /// TODO: Since we must manually add a new session, it would definitely stand to reason that we
    /// should not consume the webdriver here, or, be tied specifically to a single session
    pub fn delete_session(self) {
        let url = construct_url(vec!["session/", &self.session_id.unwrap()]);
        let _ = self.client.delete(url).send();
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
}
