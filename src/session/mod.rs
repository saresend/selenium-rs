use reqwest;

trait SessionHandler {
    fn new_session(&self, params: NewSessionParams) -> reqwest::Result<NewSessionResult>;
    fn delete_session(&self) -> reqwest::Result<()>;
}


#[derive(Serialize, Deserialize)] 
pub struct NewSessionParams {
    #[serde(rename="desiredCapabilities")]
    desired_capabilities: DesiredCapabilities, 
}

#[derive(Serialize, Deserialize)]
pub struct DesiredCapabilities {
    #[serde(rename="browserName")]
    browser_name: String, 
}


#[derive(Serialize, Deserialize)]
pub struct NewSessionResult {
    session_id: String, 
}