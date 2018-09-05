use element::Element;
use reqwest;

#[derive(Deserialize)]
pub struct SelectedResponse {
    pub value: bool,
}

#[derive(Deserialize)]
pub struct ElementResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    status: i32,
    value: ElemValue,
}

#[derive(Deserialize)]
struct ElemValue {
    #[serde(rename = "ELEMENT")]
    element_id: String,
}

impl<'a> ElementResponse {
    pub fn parse_into_element(self, client: &'a reqwest::Client) -> Element<'a> {
        Element::new(self.value.element_id, self.session_id, client)
    }
}

#[derive(Deserialize)]
pub struct AttributeResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    pub value: String,
}
