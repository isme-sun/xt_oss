use dotenv;
use std::process;
use std::{
    env, fs,
    io::{BufRead, BufReader},
};
use xt_oss::prelude::*;

async fn append_upload<'a>(
    client: &'a oss::Client<'a>,
    object: &'a str,
    position: usize,
    data: oss::Bytes,
) -> usize {
    println!("append: pos({}) size({})", &position, &data.len());
    let content_length = match client
        .AppendObject(object)
        .with_position(position)
        .with_content(data)
        .execute()
        .await
        .unwrap_or_else(|error| {
            eprintln!("reqwest error: {}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            let pos = data.headers().get("x-oss-next-append-position").unwrap();
            pos.to_str().unwrap().parse::<usize>().unwrap()
        }
        Err(message) => {
            eprintln!("iss error: {}", message.content());
            process::exit(-1);
        }
    };
    content_length
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);
    let target_scv_file = {
        let mut current_dir = env::current_dir()?;
        ["examples", "samples", "txt", "organizations-100000.csv"]
            .iter()
            .for_each(|e| current_dir.push(e));
        current_dir.to_owned().display().to_string()
    };

    let object = "tmp/append_ex.csv";

    // 如果object存在先删除
    if client.HeadObject(object).execute().await?.is_ok() {
        println!("删除已存在的文件\n");
        client
            .DeleteObject(object)
            .execute()
            .await?
            .unwrap_or_else(|error| {
                eprint!("oss error: {}", error.content());
                process::exit(-1);
            });
    }

    let scv_file = fs::File::open(target_scv_file)?;
    let mut buffer_lines = Vec::new();
    let mut position = 0;

    println!("append start..");
    for (i, line) in BufReader::new(scv_file).lines().enumerate() {
        buffer_lines.push(line?);
        if i != 0 && i % 3000 == 0 {
            let content = format!("{}\n", buffer_lines.join("\n"));
            let content = oss::Bytes::from(content);
            position = append_upload(&client, object, position, content).await;
            buffer_lines.clear();
        }
    }

    if !buffer_lines.is_empty() {
        let content = buffer_lines.join("\n");
        let content = oss::Bytes::from(content);
        let _ = append_upload(&client, object, position, content).await;
    }

    Ok(())
}
