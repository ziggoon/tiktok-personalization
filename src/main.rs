use std::thread;

mod util;

#[tokio::main]
async fn main() {
    thread::spawn(|| {
        println!("\t\t\t       starting api server!");
        util::twilio_helper::main();
    });

    util::tiktok_helper::register_user().await;
}
