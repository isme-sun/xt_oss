// use std::process;

use std::fs::File;
use std::io::Read;

use dotenv;
use xt_oss::oss;
use xt_oss::utils;

fn read_assets_file(filepath: String) -> Vec<u8> {
    let current_dir = std::env::current_dir().unwrap();
    let filepath = format!("{}/assets/{}", current_dir.to_str().unwrap(), filepath);
    let mut pic_file = File::open(filepath).unwrap();
    let mut content: Vec<u8> = Vec::new();
    pic_file.read_to_end(&mut content).unwrap();
    content
}

#[allow(unused)]
async fn put_object() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);

    let result = client
        .PutObject("xtoss/example/settings.json")
        .content({
            let filename = String::from("settings.json");
            let content = read_assets_file(filename);
            oss::Bytes::from(content)
        })
        .send()
        .await;

    match result {
        Ok(data) => println!("{:#?}", data.headers),
        Err(message) => println!("{}", message),
    }
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
    // object_list().await;
    put_object().await;
}
