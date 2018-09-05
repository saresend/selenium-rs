use element::{Element, ElementRequest};
use element_structs::ElementResponse;
use reqwest;
use session_structs::{NewSessionRequest, NewSessionResponse};
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

// Contains Navigation Handling
impl WebDriver {
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
