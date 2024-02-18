use std::{process, sync::Arc};

use dotenv;
use futures::future::join_all;
use std::io::{self, Write};
use xt_oss::{oss, utils};

async fn del_all() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = Arc::new(oss::Client::new(options));
    loop {
        match client
            .ListObjectsV2()
            .with_max_keys(5)
            .execute()
            .await
            .unwrap_or_else(|error| {
                println!("reqwest error: {}", error);
                process::exit(-1);
            }) {
            Ok(data) => {
                let data = data.content();
                if let Some(objects) = data.contents {
                    let tasks = objects.into_iter().map(|object| {
                        let client = Arc::clone(&client);
                        async move {
                            let result = client.DeleteObject(object.key.as_str()).execute().await;
                            match result {
                                Ok(Ok(_)) => println!("delete object key: {}", object.key),
                                Ok(Err(message)) => println!("{}", message.content()),
                                Err(error) => println!("reqwest error: {}", error),
                            }
                        }
                    });
                    join_all(tasks).await;
                } else {
                    println!("bucket file clean");
                    return Ok(());
                }
            }
            Err(message) => {
                println!("oss error: {}", message.content())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("警告：后续操作可能存在危险！");
    loop {
        print!("是否继续操作?(Y/n):");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_uppercase().as_str() {
            "Y" => {
                println!("继续运行...");
                del_all().await?;
                break;
            }
            "n" => {
                println!("操作已取消。");
                break;
            }
            _ => {
                println!("无效的输入，请输入 Y 或 n。");
            }
        }
    }
    Ok(())
}
