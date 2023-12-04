use std::thread;
use std::time::Duration;
use thirtyfour::{cookie::SameSite, prelude::*};

// just allows me to add a cookie for testing
pub async fn add_cookie() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://tiktok.com").await?;

    let cook = Cookie::build("sessionid", "whythefuckdidicommitthis")
        .domain(".tiktok.com")
        .path("/")
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .finish();

    driver.add_cookie(cook).await?;
    driver.refresh().await?;
    thread::sleep(Duration::from_secs(600));
    Ok(())
}
