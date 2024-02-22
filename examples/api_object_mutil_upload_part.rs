use dotenv;
use std::io::{Seek, SeekFrom};
use std::{env, fs, io::Read, os::unix::fs::MetadataExt, process};
use xt_oss::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let options = util::options_from_env();
    let client = oss::Client::new(options);

    let target_file = {
        let mut current_dir = env::current_dir()?;
        "examples/samples/images/JPGImage_30mbmb.jpg"
            .split("/")
            .for_each(|e| current_dir.push(e));
        current_dir
    };

    let mut file = fs::File::open(&target_file)?;
    let file_size = file.metadata()?.size();
    let chunk_size = 1024 * 1024;
    let object = "tmp/temp.jpg";

    let upload_id = client
        .InitiateMultipartUpload(object)
        .execute()
        .await
        .unwrap_or_else(|error| {
            eprintln!("reqwest error: {}", error);
            process::exit(-1);
        })
        .unwrap_or_else(|message| {
            eprintln!("oss error: {}", message.content());
            process::exit(-1);
        })
        .content()
        .upload_id;

    let chunks = ByteRange::chunk(file_size, chunk_size);
    let file_chunks = chunks.iter().map(|range| {
        let (seek, length) = (range.start(), range.amount() as u64);
        let mut buffer = vec![0; length as usize];
        let _ = file.seek(SeekFrom::Start(seek));
        let _ = file.read_exact(&mut buffer);
        oss::Bytes::from(buffer)
    });

    for (i, content) in file_chunks.enumerate() {
        let part_number = i + 1;
        let result = client
            .UploadPart(object)
            .with_part_number(part_number as u32)
            .with_upload_id(&upload_id)
            .with_content(content)
            .execute()
            .await
            .unwrap()
            .unwrap();
        println!("part {:#?}", result.headers())
    }

    match client
        .CompleteMultipartUpload(object)
        .with_upload_id(&upload_id)
        .with_encoding_type("url")
        .with_forbid_overwrite(false)
        .execute()
        .await
    {
        Ok(Ok(data)) => println!("{:#?}", data.content()),
        Ok(Err(message)) => println!("{:#?}", message.content()),
        Err(error) => println!("{}", error),
    }

    Ok(())
}
