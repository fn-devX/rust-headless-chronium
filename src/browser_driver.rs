use std::process::Command;
use thirtyfour::prelude::*;
use serde_json::json;
use dotenvy::dotenv;
use crate::data_access::get_env_var;

pub async fn start_driver() -> WebDriverResult<WebDriver> {
    dotenv().ok();

    let chromedriver_path = get_env_var("CHROMEDRIVER_PATH");
    let base_url = get_env_var("BASE_URL");

    Command::new(chromedriver_path)
        .arg("--port=9515")
        .spawn()
        .expect("failed to start Chromedriver");

    let mut caps = DesiredCapabilities::chrome();
    caps.set_base_capability("goog:chromeOptions", json!({
        "excludeSwitches": ["enable-automation"],
        "args": [format!("--app={}", base_url)]
    }))?;

    WebDriver::new("http://localhost:9515", caps).await
}