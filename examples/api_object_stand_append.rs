#[allow(unused)]
use dotenv;
#[allow(unused)]
use std::process;
#[allow(unused)]
use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
};
#[allow(unused)]
use xt_oss::{oss::entities::object::TaggingDirective, prelude::*};

async fn append_upload<'a>(client: &'a oss::Client<'a>, position: usize, data: oss::Bytes) -> usize {
    println!("------");
    let content_length = match client
        .AppendObject("tmp/append_ex.csv")
        .with_position(position)
        .with_content(data)
        .execute()
        .await
        .unwrap_or_else(|error| {
            println!("reqwest error: {}", error);
            process::exit(-1);
        }) {
        Ok(data) => {
            let pos = data.headers().get("x-oss-next-append-position").unwrap();
            pos.to_str().unwrap().parse::<usize>().unwrap()
        }
        Err(message) => {
            println!("iss error: {}", message.content());
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
    println!("{}", target_scv_file.display());

    let scv_file = fs::File::open(target_scv_file)?;

    let mut buffer_lines = Vec::new();

    let mut i = 0;
    let mut position = 0;
    for line in BufReader::new(scv_file).lines() {
        buffer_lines.push(line?);
        if i != 0 && i % 3000 == 0 {
            let content = format!("{}\n", buffer_lines.join("\n"));
            let content = oss::Bytes::from(content);
            println!("position {}", position);
            position = append_upload(&client, position, content).await;
            println!("return {}", position);
            buffer_lines.clear();
        }
        i = i + 1;
    }
    let content = format!("{}\n", buffer_lines.join("\n"));
    let content = oss::Bytes::from(content);
    let _ = append_upload(&client, position, content).await;

    Ok(())
}
