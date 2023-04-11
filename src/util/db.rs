use rusqlite::{Connection, Result, Error};
use pbkdf2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};

use crate::util::web_helper::User;

pub async fn check_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table if not exists users (
            id integer primary key autoincrement,
            username text not null,
            email text not null,
            dob text not null,
            password_hash blob not null,
        )",
        [],
    )?;
    Ok(())
}

pub async fn add_user() -> Result<()> {
    println!("[DATABASE] adding new user");
    //let salt = SaltString::generate(&mut OsRng);
    //let password_hash = Pbkdf2.hash_password(args[3].as_str(), &salt)?.to_string();

    /*conn.execute(
        "insert into users (fname, lname, username, password_hash) values (?1 ?2 ?3 ?4)",
        &[args[1].as_str(), args[2].as_str(), args[3].as_str(), &password_hash],
    ).expect("user insert failed");*/
    Ok(())
}
