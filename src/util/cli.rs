use clap::Command;
use colored::*;

use futures::StreamExt;
use std::io::Write;

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

pub async fn main_loop() -> Result<(), Box<dyn std::error::Error>> {
    banner();
    loop {
        let line = readline()?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match respond(line).await {
            Ok(quit) => {
                if quit {
                    break;
                }
            }
            Err(e) => {
                write!(std::io::stdout(), "{e}").map_err(|e| e.to_string())?;
                std::io::stdout().flush().map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

async fn respond(line: &str) -> Result<bool, String> {
    let args = shlex::split(line).ok_or("[error] failed to read command")?;
    let matches = cli()
        .try_get_matches_from(args)
        .map_err(|e| e.to_string())?;
    match matches.subcommand() {
        Some(("debug", _matches)) => {
            write!(
                std::io::stdout(),
                "[debug] logging into TikTok with debug account \n"
            )
            .map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            util::debug::add_cookie().await.unwrap();
        }
        Some(("start", _matches)) => {
            write!(std::io::stdout(), "[+] starting data collection \n")
                .map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;

            let db_client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
                .await
                .unwrap();
            let mut handles = vec![];

            for _ in 0..5 {
                let user = util::db::get_random_user(db_client.clone()).await.unwrap();

                let handle = tokio::spawn(async move {
                    user.run().await.unwrap();
                });

                handles.push(handle);
            }

            futures::stream::iter(handles)
                .for_each(|f| async {
                    let _ = f.await;
                })
                .await;
        }
        Some(("quit", _matches)) => {
            write!(std::io::stdout(), "Exiting ...").map_err(|e| e.to_string())?;
            std::io::stdout().flush().map_err(|e| e.to_string())?;
            return Ok(true);
        }
        Some((name, _matches)) => unimplemented!("{name}"),
        None => unreachable!("subcommand required"),
    }

    Ok(false)
}

fn cli() -> Command {
    // strip out usage
    const PARSER_TEMPLATE: &str = "\
        {all-args}
    ";
    // strip out name/version
    const APPLET_TEMPLATE: &str = "\
        {about-with-newline}\n\
        {usage-heading}\n    {usage}\n\
        \n\
        {all-args}{after-help}\
    ";

    Command::new("repl")
        .multicall(true)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand_value_name("cmd")
        .subcommand_help_heading("commands")
        .help_template(PARSER_TEMPLATE)
        .subcommand(
            Command::new("debug")
                .about("enter debug mode")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("start")
                .about("start data collection")
                .help_template(APPLET_TEMPLATE),
        )
        .subcommand(
            Command::new("quit")
                .alias("exit")
                .about("quit the program")
                .help_template(APPLET_TEMPLATE),
        )
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "algoprober# ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
