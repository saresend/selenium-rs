use element::Element;
use reqwest;

#[derive(Deserialize)]
pub struct SelectedResponse {
    pub selected: bool,
}

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
