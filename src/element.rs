/*! 

    Element enables most of the site interaction, and wraps user interactions such as typing text and clicking
    on things. Note that each element is tied to the specific session (currently, we
    can't hold on to the same element across sessions).


    # Example - Inspecting attributes of an element 

    ```rust 
        use selenium_rs::webdriver::{Selector, Browser, WebDriver};
        use selenium_rs::element::Element;

        let mut driver = WebDriver::new(Browser::Chrome);
        driver.start_session();
        driver.navigate("http://www.google.com");
        let search_form =  driver.find_element(Selector::CSS, "#searchform").unwrap();

        assert!(search_form.get_css_value("min-width").unwrap() == "980px");
    ```
*/

use element_structs::*;
use reqwest;
use utils::construct_url;

/// Element provides an interface from which to inspect and interact with the requested elements
/// on the page. The general flow involves navigating to your webpage in question, and then
/// requesting a specific element through the query API, and then using the returned element to
/// inspect the element on the page
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

    /// gets an element attribute for the given element
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

    /// retrieves the property value for the given element, if it exists
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

    /// Gets a css property for the given element, if it exists
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

    /// Gets the text for a given element, if it exists or makes sense
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

    /// Clears a content editable element's text value, or returns an error
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

    /// Tries to type into a content editable element
    pub fn type_text(&self, input: &str) -> reqwest::Result<()> {
        let url = construct_url(vec![
            "session/",
            &(self.session_id.clone() + "/"),
            "element/",
            &(self.element_id.clone() + "/"),
            "value",
        ]);

        println!("{:?}", url);

        let data = ValueRequest::new(input);
        self.client
            .post(url)
            .json(&data)
            .send()?
            .error_for_status()?;

        Ok(())
    }
}
