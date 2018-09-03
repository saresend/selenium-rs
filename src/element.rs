use reqwest;

#[derive(Deserialize)]
pub struct ElementResponse {
    #[serde(rename = "ELEMENT")]
    element_id: String,
}

impl<'a> ElementResponse {
    pub fn parse_into_element(
        self,
        session_id: String,
        client: &'a reqwest::Client,
    ) -> Element<'a> {
        Element::new(self.element_id, session_id, client)
    }
}

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
