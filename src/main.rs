mod util;
use thirtyfour::prelude::{DesiredCapabilities, WebDriver, WebDriverResult};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    util::cli::main_loop(&driver).await.unwrap();
    driver.quit().await?;
    Ok(())
}
