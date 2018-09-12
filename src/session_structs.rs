// Session Request Object
#[derive(Serialize, Deserialize)]
pub struct NewSessionRequest {
    #[serde(rename = "desiredCapabilities")]
    desired_capabilities: DesiredCapabilitiesRequest,
}

#[derive(Serialize, Deserialize)]
struct DesiredCapabilitiesRequest {
    #[serde(rename = "browserName")]
    browser_name: String,
}

impl NewSessionRequest {
    pub fn new(browser_name: &str) -> NewSessionRequest {
        NewSessionRequest {
            desired_capabilities: DesiredCapabilitiesRequest::create(browser_name.to_string()),
        }
    }
}

impl DesiredCapabilitiesRequest {
    pub fn create(browser_name: String) -> DesiredCapabilitiesRequest {
        DesiredCapabilitiesRequest { browser_name }
    }
}

// Session Response object
#[derive(Serialize, Deserialize)]
pub struct NewSessionResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
}

impl NewSessionResponse {
    pub fn get_session_id(self) -> String {
        self.session_id
    }
}

// Title Response Object
#[derive(Deserialize)]
pub struct TitleResponse {
    value: String,
}

impl TitleResponse {
    pub fn get_title(self) -> String {
        self.value
    }
}
