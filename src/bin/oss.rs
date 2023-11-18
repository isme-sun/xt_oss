use std::error::Error;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}