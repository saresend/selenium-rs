use element_structs::*;
use reqwest;
use serde_json;
use utils::construct_url;

#[derive(Serialize, Deserialize)]
pub struct ElementRequest {
    using: String,
    value: String,
}

impl ElementRequest {
    pub fn new(using: String, value: String) -> ElementRequest {
        ElementRequest { using, value }
    }
}

#[derive(Debug)]
pub struct Element<'a> {
    element_id: String,
    session_id: String,
    client: &'a reqwest::Client,
}

impl<'a> Element<'a> {
    pub fn new(element_id: String, session_id: String, client: &'a reqwest::Client) -> Element<'a> {
        Element {
            element_id,
            session_id,
            client,
        }
    }
}

// Contains implementation for attribute interaction for the element
impl<'a> Element<'a> {
    pub fn is_selected(&self) -> reqwest::Result<bool> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "selected",
        ]);
        let response: SelectedResponse = self.client.get(url).send()?.error_for_status()?.json()?;
        Ok(response.value)
    }

    pub fn get_attribute(&self, attribute: &str) -> reqwest::Result<String> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "attribute/",
            attribute,
        ]);
        let result: AttributeResponse = self.client.get(url).send()?.error_for_status()?.json()?;

        Ok(result.value)
    }

    pub fn get_property(&self, property: &str) -> reqwest::Result<String> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "property/",
            property,
        ]);
        let result: AttributeResponse = self.client.get(url).send()?.error_for_status()?.json()?;

        Ok(result.value)
    }

    pub fn get_css_value(&self, css_property: &str) -> reqwest::Result<String> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "css/",
            css_property,
        ]);
        let result: AttributeResponse = self.client.get(url).send()?.error_for_status()?.json()?;

        Ok(result.value)
    }

    pub fn get_text(&self) -> reqwest::Result<String> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "text",
        ]);
        let result: AttributeResponse = self.client.get(url).send()?.error_for_status()?.json()?;

        Ok(result.value)
    }
}

// Implements Element Interactability

impl<'a> Element<'a> {
    pub fn click(&self) -> reqwest::Result<()> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "click",
        ]);
        self.client.post(url).send()?.error_for_status()?;
        Ok(())
    }
    pub fn clear(&self) -> reqwest::Result<()> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "clear",
        ]);
        self.client.post(url).send()?.error_for_status()?;

        Ok(())
    }
    pub fn type_text(&self, input: &str) -> reqwest::Result<()> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "value",
        ]);

        let data = ValueRequest::new(input);
        self.client
            .post(url)
            .json(&data)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}
