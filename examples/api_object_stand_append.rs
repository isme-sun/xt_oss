use dotenv;
use std::process;
use std::{
    env, fs,
    io::{BufRead, BufReader},
};
#[allow(unused)]
use xt_oss::{oss::entities::object::TaggingDirective, prelude::*};

async fn append_upload<'a>(client: &'a oss::Client<'a>, position: usize, data: oss::Bytes) -> usize {
    let content_length = match client
        .AppendObject("tmp/append_ex.csv")
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
            .into_iter()
            .for_each(|e| current_dir.push(e));
        current_dir.to_owned()
    };
    let scv_file = fs::File::open(target_scv_file)?;
    let mut buffer_lines = Vec::new();
    let mut position = 0;
    for (i, line) in BufReader::new(scv_file).lines().enumerate() {
        buffer_lines.push(line?);
        if i != 0 && i % 3000 == 0 {
            let content = format!("{}\n", buffer_lines.join("\n"));
            let content = oss::Bytes::from(content);
            position = append_upload(&client, position, content).await;
            buffer_lines.clear();
        }
    }
    let content = buffer_lines.join("\n");
    let content = oss::Bytes::from(content);
    let _ = append_upload(&client, position, content).await;
    Ok(())
}
