use thirtyfour::prelude::*;

mod browser_driver;
mod actions;
mod navigation;
mod data_access;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let driver = browser_driver::start_driver().await?;

    driver.goto(&std::env::var("BASE_URL").unwrap()).await?;
    actions::perform_search(&driver).await?;

    navigation::track_url_changes(&driver).await?;
    Ok(())
}