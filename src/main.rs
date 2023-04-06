use std::thread;

mod util;

#[tokio::main]
async fn main() {
    /*thread::spawn(|| {
        println!("\t\t\t       starting api server!");
        util::twilio_helper::main();
    });*/

    util::web_helper::twitter_register_user().await.unwrap();
}
