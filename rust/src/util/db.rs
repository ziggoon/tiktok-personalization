use futures::StreamExt;
use mongodb::bson::doc;

use crate::util::user::User;

pub async fn get_random_user(
    client_ref: mongodb::Client,
) -> Result<User, Box<dyn std::error::Error>> {
    let db = client_ref.database("test");
    let collection: mongodb::Collection<User> = db.collection("users");

    let pipeline = vec![doc! { "$sample" : { "size" : 1 } }];

    if let Ok(mut cursor) = collection.aggregate(pipeline, None).await {
        if let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    let user = User::new(
                        doc.get("_id")
                            .unwrap()
                            .as_object_id()
                            .expect("failed to convert"),
                        doc.get("email").unwrap().as_str().unwrap().to_owned(),
                        doc.get("username").unwrap().as_str().unwrap().to_owned(),
                        doc.get("password").unwrap().as_str().unwrap().to_owned(),
                    );

                    return Ok(user);
                    //println!("{:?}", user);
                }
                Err(e) => {
                    eprintln!("err {}", e);
                }
            }
        }
    } else {
        eprintln!("error retrieving cursor");
    }
    Err("failed to get user :(".into())
}
