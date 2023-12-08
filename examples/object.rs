use dotenv;
use xt_oss::oss;
use xt_oss::oss::Bytes;
use xt_oss::utils;

#[allow(unused)]
async fn put_object() {
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

#[allow(unused)]
async fn object_list() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjectsV2()
        .prefix("course/video")
        .max_keys(10)
        .send()
        .await
        .unwrap();

    for item in result.data.contents {
        println!("{}", urlencoding::decode(&item.key).unwrap());
    }
}

#[tokio::main]
async fn main() {
    object_list().await;
}
