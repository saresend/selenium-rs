use reqwest;

pub trait WebDriverNavigation {
    fn navigate_to_url(&self, url: &str) -> reqwest::Result<()>;
    fn get_current_url(&self) -> reqwest::Result<String>;
    fn go_back(&self) -> reqwest::Result<()>;
    fn go_forward(&self) -> reqwest::Result<()>;
    fn refresh(&self) -> reqwest::Result<()>;
    fn get_title(&self) -> reqwest::Result<String>;
}

#[derive(Serialize, Deserialize)]
pub struct URLContainer<'a> {
    url: &'a str,
}

impl<'a> URLContainer<'a> {
    pub fn new(url: &'a str) -> URLContainer<'a> {
        URLContainer { url }
    }
}
