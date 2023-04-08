use std::time::Duration;
use std::thread;
use std::io;

use thirtyfour::prelude::*;

use rand::Rng;

#[derive(Debug)]
pub struct User {
    name: String,
    email: String,
    dob: String,
    username: String,
    password_hash: String,
}

async fn generate_password() -> String {
    let password: String = (0..16)
        .map(|_| {
            let mut random_byte;
            loop {
                random_byte = rand::thread_rng().gen::<u8>();
                if random_byte != b'"' && random_byte != b'\'' {
                    break;
                }
            }
            match random_byte % 4 {
                0 => (random_byte % 26 + b'a') as char,          // Lowercase letter
                1 => (random_byte % 26 + b'A') as char,          // Uppercase letter
                2 => (random_byte % 10 + b'0') as char,          // Digit
                _ => (random_byte % 15 + 33) as char,            // Special character (! through / and : through @ in ASCII)
            }
        })
        .collect();
    
    return password;
}

pub async fn register_user(driver: &WebDriver, args: Vec<String>) -> WebDriverResult<()> {
    let dob = format!("{} {} {}", args[3].as_str(), args[4].as_str(), args[5].as_str());
    let user = User {
        name: args[1].to_string(),
        email: args[2].to_string(),
        dob: dob,
        username: args[6].to_string(),
        password_hash: generate_password().await,
    };
    println!("{:?}", &user);

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

                    let refreshed_child_elems = driver.find_all(By::Tag("input")).await?;
                    refreshed_child_elems[0].send_keys(&user.name).await?;
                    refreshed_child_elems[1].send_keys(&user.email).await?;


                    let mut parts = user.dob.split_whitespace();

                    let day = parts.next().unwrap().to_string();
                    let month = parts.next().unwrap().to_string();
                    let year = parts.next().unwrap().to_string();

                    let dropdowns = driver.find_all(By::Tag("select")).await?;
                    dropdowns[0].send_keys(month).await?;
                    dropdowns[1].send_keys(day).await?;
                    dropdowns[2].send_keys(year).await?;

                    thread::sleep(Duration::from_secs(2));
                    let next = driver.find_all(By::Tag("span")).await?;
                    next[next.len()-1].click().await?;
                    break;
                }
            }
            break;
        }
    }

    let next = driver.find_all(By::Tag("span")).await?;
    next[next.len()-1].click().await?;

    let signup = driver.find_all(By::Tag("span")).await?;
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
