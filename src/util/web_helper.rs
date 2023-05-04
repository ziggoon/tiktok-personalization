use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use std::io;
use std::thread;
use std::time::Duration;

use colored::*;
use cookie::Cookie;
use thirtyfour::prelude::*;

use crate::util;
use crate::util::db::User;

pub async fn scroll(driver: &WebDriver) -> WebDriverResult<()> {
    let mut rng = thread_rng();
    let interval_range = Uniform::new(0.5, 10.0);

    // Iterate 10 times
    for _ in 0..10 {
        println!("{}", "[+] scrolling now..".green());
        // Scroll down using JavaScript.
        driver.execute("window.scrollBy(0, 720);", vec![]).await?;

        // Generate random interval between 0.5 and 10 seconds.
        let random_interval = rng.sample(interval_range);

        println!("sleeping for {}s", &random_interval);
        // Pause for the random interval.
        tokio::time::sleep(Duration::from_secs_f32(random_interval)).await;
    }

    Ok(())
}

pub async fn like_video(driver: &WebDriver) -> WebDriverResult<()> {
    let elements = driver.find_all(By::Tag("strong")).await?;

    for element in elements {
        //let css_value = element.get_attribute("href").await?;
        println!("Found element: {:?} {:?}", element, element.text().await?);
    }

    Ok(())
}

pub async fn login_user(driver: &WebDriver, args: Vec<String>) -> WebDriverResult<()> {
    let email: String = args[1].to_string();
    let password: String = args[2].to_string();

    driver
        .goto("https://www.tiktok.com/login/phone-or-email/email")
        .await?;

    let login_fields: &Vec<WebElement> = &driver.find_all(By::ClassName("etcs7ny1")).await?;
    let email_field: &WebElement = &login_fields[0];
    let password_field: &WebElement = &login_fields[1];

    email_field.send_keys(email).await?;
    password_field.send_keys(password).await?;

    let login_button: WebElement = driver.find(By::ClassName("e1w6iovg0")).await?;

    thread::sleep(Duration::from_secs(1));
    login_button.click().await?;

    Ok(())
}

pub async fn add_cookie(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://tiktok.com").await?;

    let cookie = Cookie::build("sessionid", "7553d9eb2d35a3fa261985da7c8a32ee")
        .domain(".tiktok.com")
        .path("/")
        .secure(true)
        .http_only(true)
        .finish();

    driver.add_cookie(cookie).await?;

    Ok(())
}
