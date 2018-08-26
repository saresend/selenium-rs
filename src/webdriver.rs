use selenium_core::utils::*;
use selenium_core::url_builder::URLBuilder;
use session::SessionHandler;
use navigation::{TitleContainer, URLContainer, WebDriverNavigation};
use session::session_structs::*;
use reqwest;
use std::collections::BTreeMap;

pub enum SupportedBrowser {
    Chrome,
}

// The primary struct through which interaction happens
pub struct WebDriver {
    browser: SupportedBrowser,
    client: reqwest::Client,
    session_id: Option<String>,
}

impl WebDriver {
    pub fn new(browser: SupportedBrowser) -> WebDriver {
        WebDriver {
            browser,
            client: reqwest::Client::new(),
            session_id: None,
        }
    }
    fn get_sess_id(&self) -> String {
        self.session_id.clone().unwrap()
    }
}

impl SessionHandler for WebDriver {
    fn new_session(&mut self, params: NewSessionParams) -> reqwest::Result<NewSessionResult> {
        let url = URLBuilder::new().add_element("session").get_url();

        println!("{}", url);
        let response: NewSessionResult = self.client.post(url).json(&params).send()?.json()?;

        self.session_id = Some(response.get_session_id());
        Ok(response)
    }
    fn delete_session(&mut self) -> reqwest::Result<()> {
        let session_id = self.get_sess_id();
        let url = URLBuilder::new().add_kv_pair("session/", &session_id).get_url();
        let _ = self.client.delete(url).send()?;
        Ok(())
    }
    fn status(&self) -> reqwest::Result<StatusResult> {
        let url = URLBuilder::new().add_element("status").get_url();
        let res: StatusResult = self.client.get(url).send()?.json()?;
        Ok(res)
    }
    fn get_timeouts(&self) -> reqwest::Result<TimeoutResult> {
        let url = URLBuilder::new()
            .add_kv_pair("session", &self.get_sess_id())
            .add_element("timeouts")
            .get_url();
        let res: TimeoutResult = self.client.get(url).send()?.json()?;
        Ok(res)
    }
    fn set_timeouts(&self, params: TimeoutResult) -> reqwest::Result<()> {
        let url = URLBuilder::new()
            .add_kv_pair("session", &self.get_sess_id())
            .add_element("timeouts")
            .get_url();
        self.client.post(url).json(&params).send()?;
        Ok(())
    }
}

impl WebDriverNavigation for WebDriver {
    fn navigate_to_url(&self, url: &str) -> reqwest::Result<()> {
        let url_val = URLBuilder::new()
            .add_element("session")
            .add_kv_pair(&self.get_sess_id(), "url")
            .get_url();
        let post_body = URLContainer::new(&url);
        self.client.post(url_val).json(&post_body).send()?;
        Ok(())
    }
    fn get_current_url(&self) -> reqwest::Result<String> {
        let url = URLBuilder::new()
            .add_element("session")
            .add_kv_pair(&self.get_sess_id(), "url")
            .get_url();

        let container: URLContainer = self.client.get(url).send()?.json()?;
        Ok(container.url)
    }
    fn go_back(&self) -> reqwest::Result<()> {
        let url = URLBuilder::new()
            .add_kv_pair("session", &self.get_sess_id())
            .add_element("back")
            .get_url();
        self.client.post(url).send()?;
        Ok(())
    }
    fn go_forward(&self) -> reqwest::Result<()> {
        let url = URLBuilder::new()
            .add_kv_pair("session", &self.get_sess_id())
            .add_element("forward")
            .get_url();
        self.client.post(url).send()?;
        Ok(())
    }
    fn refresh(&self) -> reqwest::Result<()> {
        let url = URLBuilder::new()
            .add_kv_pair("session", &self.get_sess_id())
            .add_element("refresh")
            .get_url();
        self.client.post(url).send()?;
        Ok(())
    }
    fn get_title(&self) -> reqwest::Result<String> {
        let url = URLBuilder::new()
            .add_kv_pair("session", &self.get_sess_id())
            .add_element("title")
            .get_url();
        let res: TitleContainer = self.client.post(url).send()?.json()?;
        Ok(res.title)
    }
}
