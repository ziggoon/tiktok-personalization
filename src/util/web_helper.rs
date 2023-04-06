use std::time::Duration;
use std::thread;
use std::io;

use thirtyfour::prelude::*;

pub async fn tiktok_register_user() -> WebDriverResult<()> {
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

pub async fn twitter_register_user() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
    // userAgent: 'Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.7113.93 Safari/537.36
    //caps.add_chrome_option("useAutomaticExtension", false)?;
    //caps.add_chrome_option("excludeSwitches", ["enable-automation"])?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://twitter.com/i/flow/signup").await?;
    thread::sleep(Duration::from_secs(5));

    let elems = driver.find_all(By::ClassName("css-901oao")).await?;
    for elem in &elems {
        let div = elem.text().await?;
        if div == "Create account" {
            elem.click().await?;
            thread::sleep(Duration::from_secs(1));

            let child_elems = driver.find_all(By::ClassName("css-901oao")).await?;
            for child in &child_elems {
                let div = child.text().await?;

                if div == "Use email instead" {
                    child.click().await?;
                    thread::sleep(Duration::from_secs(1));

                    let refreshed_child_elems = driver.find_elements(By::Tag("input")).await?;
                    refreshed_child_elems[0].send_keys("test user").await?;
                    refreshed_child_elems[1].send_keys("syxuffajtizietvijj@bbitj.com").await?;

                    let dropdowns = driver.find_elements(By::Tag("select")).await?;
                    dropdowns[0].send_keys("April").await?;
                    dropdowns[1].send_keys("20").await?;
                    dropdowns[2].send_keys("1969").await?;

                    thread::sleep(Duration::from_secs(2));
                    let next = driver.find_elements(By::Tag("span")).await?;
                    next[next.len()-1].click().await?;
                    break;
                }
            }
            break;
        }
    }

    let next = driver.find_elements(By::Tag("span")).await?;
    next[next.len()-1].click().await?;

    let signup = driver.find_elements(By::Tag("span")).await?;
    signup[signup.len()-1].click().await?;

    println!("complete captcha / user verification");
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");

    let verification_code = driver.find_elements(By::Tag("input")).await?;
    verification_code[0].click().await?;
    verification_code[0].send_keys("asdasd").await?;

    let next2 = driver.find_elements(By::Tag("span")).await?;
    next2[next2.len()-1].click().await?;

    thread::sleep(Duration::from_secs(15));
    driver.quit().await?;
    Ok(())
}
