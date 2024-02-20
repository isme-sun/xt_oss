use std::{env, fs, io::Write, path::PathBuf};

use dotenv;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let down_dir = {
        let base_dir = env::var("HOME").unwrap_or_else(|_| env::temp_dir().display().to_string());
        let mut down_dir = PathBuf::from(base_dir);
        down_dir.push("xtoss");
        down_dir.push("samples");
        down_dir
    };

    fs::create_dir_all(&down_dir)?;
    println!("down file to {}", down_dir.display());

    let mut token: Option<String> = None;
    loop {
        match client
            .ListObjectsV2()
            .with_max_keys(5)
            // .with_prefix("txt")
            // .with_encoding_type("url")
            .with_continuation_token(token.as_deref())
            .execute()
            .await
            .unwrap_or_else(|error| {
                println!("reqwest error: {}", error);
                std::process::exit(-1);
            }) {
            Ok(data) => {
                let objects = data.content();
                if objects.key_count == Some(0) {
                    println!("not object");
                } else {
                    token = objects.next_continuation_token.clone();
                    for object in objects.contents.unwrap() {
                        match client.GetObject(&object.key).execute().await {
                            Ok(Ok(data)) => {
                                let target_file = down_dir.clone().join(&object.key);
                                if let Some(dirname) = target_file.parent() {
                                    if !dirname.is_dir() {
                                        fs::create_dir_all(dirname)?;
                                    }
                                }
                                let mut file = fs::File::create(target_file)?;
                                // 写入内容
                                file.write_all(&data.content())?;
                                println!("down file: {}", &object.key);
                            }
                            Ok(Err(message)) => println!("oss error: {}", message.content()),
                            Err(error) => println!("reqwest oss: {}", error),
                        }
                    }
                }
            }
            Err(message) => {
                println!("oss error {:#?}:", message.content());
                break;
            }
        }
        if token.is_none() {
            break;
        }
    }

    Ok(())
}
