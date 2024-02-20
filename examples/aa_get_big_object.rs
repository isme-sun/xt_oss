use bytes::BytesMut;
use dotenv;
use reqwest::header::CONTENT_LENGTH;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process,
};
use xt_oss::{
    oss,
    utils::{self, ByteRange},
};

fn byte_range_chunk(total: usize, chunk_size: usize) -> Vec<ByteRange> {
    let mut reuslt: Vec<ByteRange> = vec![];
    let mut max_count = 0;
    for i in 0..total / chunk_size as usize {
        reuslt.push((i * chunk_size, chunk_size as isize).into());
        max_count = i;
    }

    let rest = total - ((max_count + 1) * chunk_size as usize);
    if rest != 0 {
        let start = total - rest;
        reuslt.push((start, rest as isize).into());
    }
    reuslt
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = utils::options_from_env();
    let client = oss::Client::new(options);
    let object = "images/JPGImage_30mbmb.jpg";
    let down_dir = {
        let base_dir = match dirs::home_dir() {
            Some(path) => path,
            None => std::env::temp_dir(),
        };
        let mut down_dir = PathBuf::from(base_dir);
        down_dir.push("xtoss");
        down_dir.push("down");
        down_dir
    };
    let save_path = down_dir.clone().join(object);
    println!("{}", save_path.display());

    let size = if let Ok(data) = client.GetObjectMeta(object).execute().await.unwrap_or_else(|error| {
        println!("reqwest error {}", error);
        process::exit(-1);
    }) {
        data.headers()
            .get(CONTENT_LENGTH)
            .map(|value| value.to_str().unwrap())
            .map(|value| value.parse::<usize>().unwrap())
    } else {
        None
    };
    println!("file total size: {:.2} MB", size.unwrap() as f64 / (1024 * 1024) as f64);
    let byte_range_list = byte_range_chunk(size.unwrap(), 1024 * 1024 * 2);

    let mut bytes = BytesMut::new();

    for (index, byte_range) in byte_range_list.iter().enumerate() {
        match client
            .GetObject(object)
            .with_range(byte_range.clone())
            .execute()
            .await
            .unwrap_or_else(|error| {
                println!("reqwest error: {}", error);
                process::exit(-1);
            }) {
            Ok(data) => {
                let rate = format!(
                    "complete {:5.2}%",
                    ((index + 1) as f64 / byte_range_list.len() as f64) * 100f64
                );
                if index == 0 {
                    print!("{rate}");
                    io::stdout().flush()?;
                } else {
                    print!("\r{rate}");
                    io::stdout().flush()?;
                }
                bytes.extend(data.content())
            }
            Err(message) => println!("oss error: {}", message.content()),
        }
    }

    println!();
    if let Some(dirname) = save_path.parent() {
        if !dirname.is_dir() {
            fs::create_dir_all(dirname)?;
        }
        let mut file = fs::File::create(save_path)?;
        file.write_all(&bytes)?;
        println!("save sucecess");
    }

    Ok(())
}
