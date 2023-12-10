use std::process;

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
        .unwrap_or_else(|err|{
            println!("{:#?}", err);
            process::exit(2);
        });
    println!("{:#?}", result);
}

#[allow(unused)]
async fn object_list() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjectsV2()
        .prefix("course/video1")
        .max_keys(10)
        .send()
        .await
        .unwrap();

    if let Some(contents) = result.data.contents {
        for item in contents {
            println!("{}", urlencoding::decode(&item.key).unwrap());
        }
    } else {
        println!("not exists");
    }

}

#[tokio::main]
async fn main() {
    object_list().await;
}
