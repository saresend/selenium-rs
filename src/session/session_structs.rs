
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

impl NewSessionResult {
    pub fn get_session_id(&self) -> String {
        self.session_id.clone()
    }
}

#[derive(Serialize, Deserialize)] 
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