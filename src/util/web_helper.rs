use std::time::Duration;
use std::thread;
use std::io;

use thirtyfour::prelude::*;

use crate::util;

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub dob: String,
    pub password_hash: String,
}


impl User {
    pub fn new(username: &String, email: &String, dob: String, password_hash: String) -> User {
        User {
            username: username.to_string(),
            email: email.to_string(),
            dob,
            password_hash
        }
    }
}

pub async fn register_user(driver: &WebDriver, args: Vec<String>) -> WebDriverResult<()> {
    let dob: String = format!("{} {} {}", args[3].as_str(), args[4].as_str(), args[5].as_str());
    let user: User = User::new(&args[1], &args[2], dob, util::db::generate_password().await);
    
    println!("User password: {:?}", &user.password_hash);
    util::db::add_user(&user).await.unwrap();

    driver.goto("https://twitter.com/i/flow/signup").await?;
    thread::sleep(Duration::from_secs(5));

    let elems: Vec<WebElement> = driver.find_all(By::ClassName("css-901oao")).await?;
    for elem in &elems {
        let div: String = elem.text().await?;
        if div == "Create account" {
            elem.click().await?;
            thread::sleep(Duration::from_secs(1));

            let child_elems: Vec<WebElement> = driver.find_all(By::ClassName("css-901oao")).await?;
            for child in &child_elems {
                let div: String = child.text().await?;

                if div == "Use email instead" {
                    child.click().await?;
                    thread::sleep(Duration::from_secs(1));

                    let refreshed_child_elems: Vec<WebElement> = driver.find_all(By::Tag("input")).await?;
                    refreshed_child_elems[0].send_keys(&user.username).await?;
                    refreshed_child_elems[1].send_keys(&user.email).await?;


                    let mut parts = user.dob.split_whitespace();

                    let day: String = parts.next().unwrap().to_string();
                    let month: String = parts.next().unwrap().to_string();
                    let year: String = parts.next().unwrap().to_string();

                    let dropdowns: Vec<WebElement> = driver.find_all(By::Tag("select")).await?;
                    dropdowns[0].send_keys(month).await?;
                    dropdowns[1].send_keys(day).await?;
                    dropdowns[2].send_keys(year).await?;

                    thread::sleep(Duration::from_secs(2));
                    let next: Vec<WebElement> = driver.find_all(By::Tag("span")).await?;
                    next[next.len()-1].click().await?;
                    break;
                }
            }
            break;
        }
    }

    let next: Vec<WebElement> = driver.find_all(By::Tag("span")).await?;
    next[next.len()-1].click().await?;

    let signup: Vec<WebElement> = driver.find_all(By::Tag("span")).await?;
    signup[signup.len()-1].click().await?;

    println!("complete captcha / user verification");
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");

    thread::sleep(Duration::from_secs(5));

    
    driver.goto("https://www.tiktok.com/en/").await?;

    println!("complete login to tiktok");
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut String::new()).expect("Failed to read line");

    Ok(())
}
