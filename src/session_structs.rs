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
