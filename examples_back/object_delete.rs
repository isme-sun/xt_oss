use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let option = utils::options_from_env();
    let client = oss::Client::new(option);
    let result = client
        .DeleteObject("xtoss/example/object2.json")
        // .with_version_id("abcd")
        .send()
        .await;
    match result {
        Ok(result) => println!("{:#?}", result),
        Err(message) => println!("{}", serde_json::to_string_pretty(&message).unwrap()),
    }
}
