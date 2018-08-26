

// Defines the implementation API for URL builder,
// and also verifies its behavior.

use url::Url;

// Do not modify imports internally to selenium
use selenium_core::url_builder::*;

fn create_url_unsafe(url_str: &str) -> Url {
    Url::parse(url_str).unwrap()
}

#[test]
fn test_default_url() {
    let url_builder = URLBuilder::new();
    assert_eq!(url_builder.get_url(), create_url_unsafe("http://localhost:4444/wd/hub/"));
}