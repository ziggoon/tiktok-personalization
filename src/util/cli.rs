use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use colored::*;
use thirtyfour::WebDriver;

use crate::util;

fn banner() {
    let banner = r#"                                                                       
    |    o|    |         |                                          |    o          |    o          
    |--- .|__/ |--- ,---.|__/    ,---.,---.,---.,---.,---.,---.,---.|    .,---,,---.|--- .,---.,---.
    |    ||  \ |    |   ||  \ ---|   ||---'|    `---.|   ||   |,---||    | .-' ,---||    ||   ||   |
    `---'``   ``---'`---'`   `   |---'`---'`    `---'`---'`   '`---^`---'`'---'`---^`---'``---'`   '
                                 |                                                                  
    "#;
    println!("{}", banner.red());
}
fn main_help() {
    let help = r#"                      
                COMMANDS

    create_user         creates new user on twitter & tiktok (password will be auto-generated)
                            usage: create_user <username> <email> <month> <day> <year>

    scroll              scrolls down the page and sleeps for a random interval
                            usage: sleep

    get                 retrieves all users stored in db
                            usage: get
    
    get_by_id           retrieves user by id
                            usage: get_by_id <id>
        
    help                this page lol

    exit                exits the program
    "#;
    println!("{}", help);
}

fn get_string_vec(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![String::from("")];
    }
    s.split_whitespace().map(str::to_string).collect()
}

pub async fn main_loop(driver: &WebDriver) -> Result<()> {
    banner();
    let db_client = mongodb::Client::with_uri_str("mongodb://localhost:27017").await.unwrap();
    
    let mut user_input: Vec<String>;
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("tiktok-test# ");
        match readline {
            Ok(line) => {
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "login_user" => util::web_helper::login_user(&driver, user_input).await.unwrap(),
                    "add_cookie" => util::web_helper::add_cookie(&driver).await.unwrap(),
                    "search" => util::web_helper::search(&driver, user_input[1].to_string()).await.unwrap(),
                    "hashtag" => util::web_helper::navigate_to_hashtag(&driver, user_input[1].to_string()).await.unwrap(),
                    "scroll" => util::web_helper::scroll(&driver).await.unwrap(),
                    "like" => util::web_helper::like_video(&driver).await.unwrap(),
                    "load_users" => util::db::load_users(db_client.clone()).await.unwrap(),
                    "help" => main_help(),
                    "exit" => break,
                    _ => continue,
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c pressed. quitting now..");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("ctrl+d pressed. quitting now..");
                break;
            }
            Err(err) => {
                println!("error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
