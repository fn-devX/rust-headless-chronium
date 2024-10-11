use std::process::Command;
use thirtyfour::prelude::*;
use serde_json::json;

pub async fn start_driver() -> WebDriverResult<WebDriver> {
    Command::new("C:\\chromedriver\\chromedriver.exe")
        .arg("--port=9515")
        .spawn()
        .expect("failed to start Chromedriver");

    let mut caps = DesiredCapabilities::chrome();
    caps.set_base_capability("goog:chromeOptions", json!({
        "excludeSwitches": ["enable-automation"],
        "args": ["--app=https://wikipedia.org"]
    }))?;

    WebDriver::new("http://localhost:9515", caps).await
}