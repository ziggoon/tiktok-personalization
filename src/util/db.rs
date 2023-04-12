use rusqlite::{Connection, Result, params};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::thread_rng;
use rand::Rng;

#[derive(Debug)]
pub struct User {
    pub id: u8,
    pub username: String,
    pub email: String,
    pub dob: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: &String, email: &String, dob: String, password_hash: String) -> User {
        User {
            id: 0,
            username: username.to_string(),
            email: email.to_string(),
            dob,
            password_hash
        }
    }
}

pub async fn check_db() -> Result<()> {
    let conn: Connection = Connection::open("tiktok.db").expect("connection failed");
    conn.execute(
        "create table if not exists users (
            id integer primary key autoincrement,
            username text not null,
            email text not null,
            dob text not null,
            password_hash text not null
        )",
        [],
    )?;
    Ok(())
}

pub async fn generate_password() -> String {
    let chars: String = concat!(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
        "0123456789",
        "!@#$%&*").to_string();

    // take the big string of characters
    // and convert it to an array of bytes
    let charset: &[u8] = &chars.into_bytes();

    fn get_random_char(charset: &[u8]) -> char {
        let idx = thread_rng().gen_range(0..charset.len());
        
        // the last statement of a Rust function (without
        // a semicolon) is the return value
        charset[idx] as char
    }
    let length: usize = 16;

    let pass: String = (0..length)
        .map(|_| get_random_char(&charset))
        .collect();

    return pass
}


pub async fn add_user(user: &User) -> Result<()> {
    let conn: Connection = Connection::open("tiktok.db").expect("connection failed");
    println!("[DATABASE] adding new user");
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(user.password_hash.as_bytes(), &salt).unwrap().to_string();

    conn.execute(
        "insert into users (username, email, dob, password_hash) values (?1, ?2, ?3, ?4)",
        [&user.username, &user.email, &user.dob, &password_hash],
    ).expect("user insert failed");
    Ok(())
}

pub async fn get_users() -> Result<()> {
    let conn: Connection = Connection::open("tiktok.db").expect("connection failed");
    let mut stmt = conn.prepare("select * from users")?;
    let rows = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            dob: row.get(3)?,
            password_hash: row.get(4)?,
        })
    })?;

    for user in rows {
        println!("found user {:?}", user.unwrap())
    }
    Ok(())
}

pub async fn get_user_by_id(args: Vec<String>) -> Result<()> {
    let conn: Connection = Connection::open("tiktok.db").expect("connection failed");

    let id = args[1].as_str();
    let mut stmt = conn.prepare("select * from users where id = ?1")?;
    let rows = stmt.query_map(params![id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            dob: row.get(3)?,
            password_hash: row.get(4)?,
        })
    })?;

    for user in rows {
        println!("{:?}", user);
    }

    Ok(())
}