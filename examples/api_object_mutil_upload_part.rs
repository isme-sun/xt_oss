#[allow(unused)]
use std::{env, fs, io::Read, os::unix::fs::MetadataExt, process};
use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    thread::sleep,
    time::Duration,
};
use xt_oss::oss::http;
#[allow(unused)]
use xt_oss::{
    oss,
    util::{self, ByteRange},
};

#[allow(unused)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let _options = util::options_from_env();
    let _client = oss::Client::new(_options);
    
    let target_file = {
        let mut current_dir = env::current_dir()?;
        "examples/samples/images/JPGImage_30mbmb.jpg"
            .split("/")
            .for_each(|e| current_dir.push(e));
        current_dir
    };

    let mut file = fs::File::open(&target_file)?;
    let file_size = file.metadata()?.size() as usize;
    let chunk_size = 1024 * 1024usize;

    let mut file1 = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("example.jpg")?;

    for byte_range in ByteRange::chunk(file_size, chunk_size) {
        let seek = byte_range.start();
        let length = byte_range.amount() as u64;
        let mut buffer = vec![0; length.try_into()?];
        sleep(Duration::from_millis(100));
        println!("{}", &seek);
        file.seek(SeekFrom::Start(seek as u64));
        file.read_exact(&mut buffer);
        file1.write_all(&buffer);
    }
    println!("{}", file_size);

    // match client
    //     .InitiateMultipartUpload("tmp/test1.png")
    //     .with_content_type("image/png")
    //     .execute()
    //     .await
    //     .unwrap_or_else(|reqwest_error| {
    //         eprintln!("{}", reqwest_error);
    //         process::exit(-1);
    //     }) {
    //     Ok(oss_data) => {
    //         println!("{:#?}", oss_data.content())
    //     }
    //     Err(error_message) => {
    //         println!("{:#?}", error_message.content())
    //     }
    // }
    Ok(())
}
