use element::Element;
use reqwest;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SelectedResponse {
    pub value: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ElementResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    status: i32,
    value: ElemValue,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ElementsResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    status: i32,
    value: Vec<ElemValue>,
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

impl<'a> ElementsResponse {
    pub fn parse_into_elements(self, client: &'a reqwest::Client) -> Vec<Element<'a>> {
        let session_id = self.session_id;
        self.value
            .into_iter()
            .map(|value| Element::new(value.element_id, session_id.clone(), client))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AttributeResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct ValueRequest<'a> {
    value: Vec<&'a str>,
}

impl<'a> ValueRequest<'a> {
    pub fn new(text: &'a str) -> ValueRequest<'a> {
        ValueRequest { value: vec![text] }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExecuteScriptResponse<T> {
    #[serde(rename = "sessionId")]
    session_id: String,
    status: i32,
    pub value: T,
}
