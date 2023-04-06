use std::time::Duration;
use std::thread;

use thirtyfour::prelude::*;

pub async fn register_user() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
    // userAgent: 'Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.7113.93 Safari/537.36
    //caps.add_chrome_option("useAutomaticExtension", false)?;
    //caps.add_chrome_option("excludeSwitches", ["enable-automation"])?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver.goto("https://www.tiktok.com/signup/phone-or-email/phone").await?;
    thread::sleep(Duration::from_secs(5));
    // Find the element you want to wait for.
    //let month = driver.query(By::ClassName("tiktok-13o6q3w-DivSelectLabel")).wait(Duration::from_secs(10), Duration::from_millis(500));
    
    let elems = driver.find_all(By::ClassName("tiktok-13o6q3w-DivSelectLabel")).await?;

    // Print the number of elements found
    println!("Found {} div elements for month,day,year", elems.len());

    for elem in &elems {
        let div = elem.text().await?;
        elem.click().await?;

        if div == "Month" {
            // Select an option by value
            let month = elem.find(By::Id("Month-options-item-3")).await?;
            month.click().await?;
        }

        if div == "Day" {
            // Select an option by value
            let day = elem.find(By::Id("Day-options-item-19")).await?;
            day.click().await?;
        }

        if div == "Year" {
            // Select an option by value
            let year = elem.find(By::Id("Year-options-item-53")).await?;
            year.click().await?;
        }
    }
    
    let phone_number = driver.find(By::ClassName("tiktok-af1p2k-InputContainer")).await?;
    phone_number.send_keys("1234567890").await?;

    thread::sleep(Duration::from_secs(10));

    // Always explicitly close the browser.
    driver.quit().await?;

    Ok(())
}