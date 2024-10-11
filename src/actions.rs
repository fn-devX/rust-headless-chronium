use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

pub async fn perform_search(driver: &WebDriver) -> WebDriverResult<()> {
    let form = driver.find(By::Id("search-form")).await?;
    sleep(Duration::from_secs(1)).await;

    let input = form.find(By::XPath("//input[@type='search']")).await?;
    input.send_keys("test").await?;
    sleep(Duration::from_secs(1)).await;

    let button = form.find(By::XPath("//button[@type='submit']")).await?;
    button.click().await?;
    driver.find(By::ClassName("firstHeading")).await?;

    Ok(())
}