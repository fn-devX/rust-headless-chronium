use std::process::Command;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio::time;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let _chromedriver = Command::new("C:\\chromedriver\\chromedriver.exe") // укажите свой путь к chromedriver
        .arg("--port=9515")
        .spawn()
        .expect("failed to start Chromedriver");

    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless=new")?; // убирает заголовок браузера

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://wikipedia.org").await?;
    let mut current_url = driver.current_url().await?;

    let form = driver.find(By::Id("search-form")).await?;

    time::sleep(Duration::from_secs(1)).await;

    let input = form.find(By::XPath("//input[@type='search']")).await?;
    input.send_keys("vnukovo").await?;

    time::sleep(Duration::from_secs(1)).await;

    let button = form.find(By::XPath("//button[@type='submit']")).await?;
    button.click().await?;

    driver.find(By::ClassName("firstHeading")).await?;

    println!("Current Url: {}", current_url);

    loop {
        tokio::time::sleep(Duration::from_millis(500)).await;

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
        // оставшуюся логику, можно будет сделать после того, как узнать, что именно требуется
    }

    // driver.quit().await?;

    // chromedriver.kill().expect("failed");
}
