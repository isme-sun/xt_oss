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
        Err(message) => println!("{:#?}", message),
    }
}

#[allow(unused)]
async fn object_list1() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjects()
        .prefix("course/video")
        .max_keys(10)
        .send()
        .await;

    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string(&data.data).unwrap())
        }
        Err(message) => println!("{}", message),
    }
}

#[allow(unused)]
async fn object_list2() {
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
    println!("{:#?}", result.data);
}

#[allow(unused)]
async fn object_put_tagging() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let object = "xtoss/example/settings.json";
    let result = client
        .PutObjectTagging(object)
        .add_tag("name", "设置文件")
        .add_tag("version", "1.0")
        .send()
        .await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
async fn object_get_tagging() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let object = "xtoss/example/settings.json";
    let result = client.GetObjectTagging(object).await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}

#[allow(unused)]
async fn object_delete_tagging() {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let object = "xtoss/example/settings.json";
    let result = client.DeleteObjectTagging(object).send().await;
    match result {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(message) => {
            println!("{:#?}", message);
        }
    }
}
#[tokio::main]
async fn main() {
    // object_list1().await;
    // put_object().await;
    // object_put_tagging().await;
    object_get_tagging().await;
    // object_delete_tagging().await;
}
