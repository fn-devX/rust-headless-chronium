use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

pub async fn track_url_changes(driver: &WebDriver) -> WebDriverResult<()> {
    let mut current_url = driver.current_url().await?;

    loop {
        sleep(Duration::from_millis(500)).await;
        let new_url = driver.current_url().await?;

        if new_url != current_url {
            if let Some(host) = new_url.host_str() {
                if !host.contains("wikipedia.org") {
                    println!("URL is tried to change to: {}", new_url);
                    println!("Out of website. Going back.");
                    driver.back().await?;
                }
            }
            current_url = new_url;
        }
    }
}