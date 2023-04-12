use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
//use rusqlite::{Connection, Result};
//use std::thread;

use thirtyfour::WebDriver;

use crate::util;

fn main_help() {
    let help = r#"                      
                                COMMANDS
                create_user         creates new user on twitter & tiktok (password will be auto-generated)
                                    usage: create_user <username> <email> <month> <day> <year> 
                help                this page lol
                quit                exits the program"#;
    println!("{}", help);
}

fn get_string_vec(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![String::from("")];
    }
    s.split_whitespace().map(str::to_string).collect()
}

pub async fn main_loop(driver: &WebDriver) -> Result<()> {
    util::db::check_db().await.unwrap();

    main_help();
    let mut user_input: Vec<String>;
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("tiktok-test# ");
        match readline {
            Ok(line) => {
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "create_user" => {
                        util::web_helper::register_user(&driver, user_input).await.unwrap();
                    },
                    "help" => main_help(),
                    "exit" => std::process::exit(0),
                    _ => continue,
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c pressed. quitting now..");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("ctrl+d pressed. quitting now..");
                break
            },
            Err(err) => {
                println!("error: {:?}", err);
                break
            }
        } 
    }
    Ok(())
}