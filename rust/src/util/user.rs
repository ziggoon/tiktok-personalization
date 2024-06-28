use serde::{Deserialize, Serialize};

use rand::Rng;
use std::thread;
use std::time::Duration;

use thirtyfour::prelude::*;

use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id_: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub type_: AccountType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountType {
    Passive,
    Active,
    Control,
}

impl User {
    pub fn new(id_: ObjectId, email: String, username: String, password: String) -> User {
        User {
            id_,
            email: email.to_string(),
            username: username.to_string(),
            password: password.to_string(),
            type_: {
                let options = vec![
                    AccountType::Passive,
                    AccountType::Active,
                    AccountType::Control,
                ];
                let mut rng = rand::thread_rng();
                let choice = rng.gen_range(0..3);
                match options[choice] {
                    AccountType::Passive => {
                        println!("[!] this account has been selected as Passive");
                        AccountType::Passive
                    }
                    AccountType::Active => {
                        println!("[!] this account has been selected as Active");
                        AccountType::Active
                    }
                    AccountType::Control => {
                        println!("[!] this account has been selected as Control");
                        AccountType::Control
                    }
                }
            },
        }
    }

    pub async fn scroll(&self, driver: WebDriver) -> WebDriverResult<()> {
        driver
            .action_chain()
            .send_keys("\u{e015}")
            .perform()
            .await?;
        Ok(())
    }

    pub async fn like_video(&self, driver: WebDriver) -> WebDriverResult<()> {
        driver.action_chain().send_keys("L").perform().await?;
        Ok(())
    }

    pub async fn add_comment(&self, driver: WebDriver) -> WebDriverResult<()> {
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

    pub async fn login(&self, driver: WebDriver) -> WebDriverResult<()> {
        driver
            .goto("https://www.tiktok.com/login/phone-or-email/email")
            .await?;

        let login_fields: &Vec<WebElement> = &driver.find_all(By::ClassName("etcs7ny1")).await?;
        let email_field: &WebElement = &login_fields[0];
        let password_field: &WebElement = &login_fields[1];

        email_field.send_keys(&self.email).await?;
        thread::sleep(Duration::from_secs(1));
        password_field.send_keys(&self.password).await?;

        let login_button: WebElement = driver.find(By::ClassName("e1w6iovg0")).await?;

        thread::sleep(Duration::from_secs(1));
        login_button.click().await?;

        Ok(())
    }

    // Site Navigation Methods (search, hashtags, ...)

    pub async fn navigate_to_hashtag(
        &self,
        driver: WebDriver,
        hashtag: String,
    ) -> WebDriverResult<()> {
        driver
            .goto(format!("https://tiktok.com/tag/{}", hashtag))
            .await?;
        Ok(())
    }

    pub async fn search(&self, driver: WebDriver, query: String) -> WebDriverResult<()> {
        driver
            .goto(format!("https://www.tiktok.com/search?q={}", query))
            .await?;
        Ok(())
    }

    pub async fn run(&self) -> WebDriverResult<()> {
        let mut caps = DesiredCapabilities::chrome();
        caps.add_chrome_arg("--disable-blink-features=AutomationControlled")?;
        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        match &self.type_ {
            AccountType::Passive => {
                println!("{}", self.username);
            }
            AccountType::Active => {
                println!("{}", self.username);
            }
            AccountType::Control => {
                println!("{}", self.username);
            }
        }
        thread::sleep(Duration::from_secs(60));
        driver.quit().await.unwrap();
        Ok(())
    }
}
