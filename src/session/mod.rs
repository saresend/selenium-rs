use reqwest;
pub mod session_structs;
use self::session_structs::*;

pub trait SessionHandler {
    fn new_session(&mut self, params: NewSessionParams) -> reqwest::Result<NewSessionResult>;
    fn delete_session(&mut self) -> reqwest::Result<()>;
    fn status(&self) -> reqwest::Result<StatusResult>;
    fn get_timeouts(&self) -> reqwest::Result<TimeoutResult>;
    fn set_timeouts(&self, params: TimeoutResult) -> reqwest::Result<()>;
}
