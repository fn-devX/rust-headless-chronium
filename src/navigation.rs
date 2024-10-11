use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};
use crate::data_access::get_env_var;

pub async fn track_url_changes(driver: &WebDriver) -> WebDriverResult<()> {
    let mut current_url = driver.current_url().await?;

    let main_url = get_env_var("BASE_URL").replace("https://", "").replace("http://", "");

    loop {
        sleep(Duration::from_millis(500)).await;
        let new_url = driver.current_url().await?;

        if new_url != current_url {
            if let Some(host) = new_url.host_str() {
                if !host.contains(&main_url) {
                    println!("URL is tried to change to: {}", new_url);
                    println!("Out of website. Going back.");
                    driver.back().await?;
                }
            }
            current_url = new_url;
        }
    }
}