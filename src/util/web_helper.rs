use colored::*;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use thirtyfour::{actions::KeyAction, cookie::SameSite, prelude::*};

use mongodb::bson::oid::ObjectId;

use serde::{Deserialize, Serialize};
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: ObjectId, email: String, username: String, password: String) -> User {
        User {
            _id: id,
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }

    pub async fn scroll(&self, driver: &WebDriver) -> WebDriverResult<()> {
        driver
            .action_chain()
            .send_keys("\u{e015}")
            .perform()
            .await?;
        Ok(())
    }

    pub async fn like_video(&self, driver: &WebDriver) -> WebDriverResult<()> {
        driver.action_chain().send_keys("L").perform().await?;
        Ok(())
    }

    pub async fn add_comment(&self, driver: &WebDriver) -> WebDriverResult<()> {
        let elements = driver.find_all(By::Tag("strong")).await?;

        for element in elements {
            if element
                .text()
                .await?
                .chars()
                .next()
                .map_or(false, |c| c.is_numeric())
                == true
            {
                let e2e_item_value = element.attr("data-e2e").await?;
                if e2e_item_value.unwrap() == "comment-count" {
                    println!("Clicking.. {}", element.text().await?);
                    element.click().await?;

                    thread::sleep(Duration::from_secs(1));

                    let input_element = driver
                        .find_all(By::ClassName("public-DraftStyleDefault-block"))
                        .await?;

                    input_element[0].click().await?;
                    input_element[0]
                        .send_keys("epic comment" + Key::Enter)
                        .await?;

                    println!("posting comment");
                }
            }
        }
        Ok(())
    }

    pub async fn login_user(&self, driver: &WebDriver) -> WebDriverResult<()> {
        driver
            .goto("https://www.tiktok.com/login/phone-or-email/email")
            .await?;

        let login_fields: &Vec<WebElement> = &driver.find_all(By::ClassName("etcs7ny1")).await?;
        let email_field: &WebElement = &login_fields[0];
        let password_field: &WebElement = &login_fields[1];

        email_field.send_keys(self.email).await?;
        password_field.send_keys(self.password).await?;

        let login_button: WebElement = driver.find(By::ClassName("e1w6iovg0")).await?;

        thread::sleep(Duration::from_secs(1));
        login_button.click().await?;

        Ok(())
    }

    // Site Navigation Methods (search, hashtags, ...)

    pub async fn navigate_to_hashtag(
        &self,
        driver: &WebDriver,
        hashtag: String,
    ) -> WebDriverResult<()> {
        driver
            .goto(format!("https://tiktok.com/tag/{}", hashtag))
            .await?;
        Ok(())
    }

    pub async fn search(&self, driver: &WebDriver, query: String) -> WebDriverResult<()> {
        driver
            .goto(format!("https://www.tiktok.com/search?q={}", query))
            .await?;
        Ok(())
    }
}

// just allows me to add a cookie for testing
pub async fn add_cookie(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://tiktok.com").await?;

    let cook = Cookie::build("sessionid", "fbc16c7c0a0926982c23d9714da9d347")
        .domain(".tiktok.com")
        .path("/")
        .same_site(SameSite::None)
        .secure(true)
        .http_only(true)
        .finish();

    driver.add_cookie(cook).await?;
    driver.refresh().await?;

    Ok(())
}
