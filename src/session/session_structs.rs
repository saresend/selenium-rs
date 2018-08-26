
#[derive(Serialize, Deserialize)] 
pub struct NewSessionParams {
    #[serde(rename="desiredCapabilities")]
    pub desired_capabilities: DesiredCapabilities, 
}

#[derive(Serialize, Deserialize)]
pub struct DesiredCapabilities {
    #[serde(rename="browserName")]
    pub browser_name: String, 
}

#[derive(Serialize, Deserialize)]
pub struct NewSessionResult {
    #[serde(rename="sessionId")]
    session_id: String, 
}

impl NewSessionResult {
    pub fn get_session_id(&self) -> String {
        self.session_id.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    value: StatusResult,
}

#[derive(Serialize, Deserialize, Debug)] 
pub struct StatusResult {
    ready: bool, 
    message: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimeoutResult {
    script: i32, 
    #[serde(rename="pageLoad")]
    page_load: i32, 
    implicit: i32,
}