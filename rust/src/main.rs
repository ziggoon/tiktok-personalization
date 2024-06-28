mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    util::cli::main_loop().await.unwrap();
    Ok(())
}
