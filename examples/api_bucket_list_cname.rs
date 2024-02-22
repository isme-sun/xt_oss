use dotenv;
use std::process;
use xt_oss::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client.ListCname().execute().await.unwrap_or_else(|error| {
        // 请求错误
        println!("reqwest error: {}", &error);
        if let Some(status) = error.status() {
            println!("is_client_error: {}", status.is_client_error());
            println!("is_informational: {}", status.is_informational());
            println!("is_redirection: {}", status.is_redirection());
            println!("is_server_error: {}", status.is_server_error());
        }
        process::exit(-1);
    });

    match result {
        Ok(data) => {
            println!("request id: {}", data.request_id());
            println!("{:#?}", data.headers());
            let content = data.content();
            println!("{:#?}", &content);
            println!("{}", serde_json::to_string_pretty(&content).unwrap())
        }
        Err(error) => {
            println!("request id: {}", error.request_id());
            println!("{:#?}", error.headers());
            println!("{:#?}", error.content())
        }
    }
}
