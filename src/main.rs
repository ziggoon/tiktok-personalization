mod util;
use thirtyfour::prelude::{ WebDriver, WebDriverResult, DesiredCapabilities };

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
    caps.add_chrome_arg("--user-agent=Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.7113.93 Safari/537.36")?;
    // userAgent: 'Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.7113.93 Safari/537.36
    //caps.add_chrome_option("useAutomaticExtension", false)?;
    //caps.add_chrome_option("excludeSwitches", ["enable-automation"])?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    util::cli::main_loop(&driver).await.unwrap();
    driver.quit().await?; 
    Ok(())
}
