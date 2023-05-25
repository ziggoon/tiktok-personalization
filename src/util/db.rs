use bcrypt::{hash, verify, DEFAULT_COST};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, bson::Document, options::FindOptions};

use crate::util::web_helper::User;

pub async fn load_users(client_ref: mongodb::Client) -> Result<(), Box<dyn std::error::Error>> {
    let db = client_ref.database("tiktok-test");
    let collection: mongodb::Collection<User> = db.collection("users");

    let find_options = FindOptions::builder().sort(doc! { "email": 1 }).build();
    let mut cursor = collection.find(None, find_options).await?;

    while let Some(item) = cursor.try_next().await? {
        println!("{}", item.email);
    }

    Ok(())
}