use colored::*;
use thirtyfour::prelude::*;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

use std::io;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: String, email: String, username: String, password: String) -> User {
        User {
            _id: id.to_string(),
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}


// User Functions

pub async fn scroll(driver: &WebDriver) -> WebDriverResult<()> {
    let mut rng = thread_rng();
    let interval_range = Uniform::new(1.0, 2.0);

    // Iterate 10 times
    for _ in 0..1 {
        println!("{}", "[+] scrolling now..".green());
        // Scroll down using JavaScript.
        driver.execute("window.scrollBy(0, 650);", vec![]).await?;

        // Generate random interval between 1 and 10 seconds.
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
        if element.text().await?.chars().next().map_or(false, |c| c.is_numeric()) == true {
            let e2e_item_value = element.get_attribute("data-e2e").await?;
            if e2e_item_value.unwrap() == "like-count" {
                println!("Clicking.. {}", element.text().await?);
                element.click().await?;
                scroll(driver).await?;
                println!("scrolling 650 pixels");
            }
        }
    }

    Ok(())
}

pub async fn login_user(driver: &WebDriver, args: Vec<String>) -> WebDriverResult<()> {
    let email: String = args[1].to_string();
    let password: String = args[2].to_string();

    driver.goto("https://www.tiktok.com/login/phone-or-email/email").await?;

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

// Site Navigation Methods (search, hashtags, ...)

pub async fn navigate_to_hashtag(driver: &WebDriver, hashtag: String) -> WebDriverResult<()> {
    driver.goto(format!("https://tiktok.com/tag/{}", hashtag)).await?;
    Ok(())
}

pub async fn search(driver: &WebDriver, query: String) -> WebDriverResult<()> {
    driver.goto(format!("https://www.tiktok.com/search?q={}", query)).await?;
    Ok(())
}


// just allows me to add a cookie for testing
pub async fn add_cookie(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://tiktok.com").await?;
    Ok(())
}
