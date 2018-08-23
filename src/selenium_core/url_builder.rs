use url::Url;


pub struct URLBuilder {
    current_url: Url,
}


impl URLBuilder {
    pub fn new() -> Self {
        URLBuilder {
            current_url: get_default_url()
        }
    }

    pub fn add_element(&mut self, token: &str) -> &mut Self {
        self.current_url = self.current_url.join(token).unwrap();
        self
    }

    pub fn add_kv_pair(&mut self, f_token: &str, s_token: &str) -> &mut Self {
        self.add_element(f_token);
        self.add_element(s_token);
        self
    }

    pub fn get_url(&self) -> Url {
        self.current_url.clone()
    }
}


fn get_default_url() -> Url {
    Url::parse("http://localhost:4444/wd/hub").unwrap()
}
