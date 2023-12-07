use xt_oss::oss;
use xt_oss::oss::Bytes;
use xt_oss::utils;
use dotenv;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .PutObject("xtoss/example/123.txt")
        .content(Bytes::from("相见时难别亦难"))
        .headers(oss::HeaderMap::new())
        .send()
        .await
        .unwrap();
    println!("{:#?}", result);
}
