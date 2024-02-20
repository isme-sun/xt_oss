use dotenv;
use std::process;
use xt_oss::{oss, util};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let result = client
        .ListObjectVersions()
        // .with_max_keys(20)
        // .with_delimiter("/")
        // .with_prefix("res/")
        // .with_encoding_type("url")
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        });

    match result {
        Ok(data) => {
            let content = data.content();
            // println!("{:#?}", content);
            println!("{}", serde_json::to_string_pretty(&content).unwrap())
            // println!("{}",content.prefix);
            // println!("FILE LIST:");
            // if let Some(delete_marker) = content.delete_marker {
            //   println!("{:#?}", delete_marker);
            // }
            // if let Some(version) = content.version {
            //   println!("{:#?}", version);
            // }

            // println!("DIR LIST:");
            // if let Some(common_prefixes) = content.common_prefixes {
            //   for item in common_prefixes {
            //     println!(" {}",item.prefix);
            //   }
            // }
        }
        Err(error) => {
            println!("{}", error.url());
            println!("{:#?}", error.content());
        }
    }
}
