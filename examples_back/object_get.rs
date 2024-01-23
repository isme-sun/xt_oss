use xt_oss::{oss, utils};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let option = utils::options_from_env();
    let client = oss::Client::new(option);
    let result = client
        .GetObject("xtoss/example/object1.json")
        .with_content_encoding("utf8")
        .with_content_type("text/xml")
        .with_content_language("zh")
        .with_cache_control("public")
        .with_content_disposition(r#"attachment;filename="filename.jpg""#)
        .with_version_id("AEQMxiBgMCfqaWA0BYiIDliMWI4MGQ0MTVmMjQ3MmE5MDNlMmY4YmFkYTk3")
        .send()
        .await;
    match result {
        Ok(data) => println!("{:#?}", data),
        Err(message) => println!("{}", serde_json::to_string_pretty(&message).unwrap()),
    }

    // let mut str_list = vec![
    //     "version_id",
    //     "content_encoding",
    //     "content_type",
    //     "content_language",
    //     "expires",
    //     "cache_control",
    //     "content_disposition"
    // ];

    // str_list.sort();
    // println!("{:#?}", str_list);
}
