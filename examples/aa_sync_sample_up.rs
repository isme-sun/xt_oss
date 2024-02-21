use std::{env, fs::File, io::Read};
use walkdir::{DirEntry, WalkDir};
use xt_oss::{oss, util};

fn only_file(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| entry.depth() == 0 || !s.starts_with("."))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let option = util::options_from_env();
    let client = oss::Client::new(option);

    let basedir = env::current_dir()?;
    let mut sampledir = basedir.clone();
    sampledir.push("examples");
    sampledir.push("samples");

    if !sampledir.is_dir() {
        panic!("not exists")
    }

    for entry in WalkDir::new(&sampledir)
        .into_iter()
        .filter_entry(|e| only_file(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path().to_string_lossy();
        let object = file_path
            .chars()
            .skip(sampledir.to_string_lossy().len() + 1)
            .collect::<String>(); // Extract object path

        let mime = match mime_guess::from_path(&object).first() {
            Some(mime) => mime.to_string(),
            None => oss::DEFAULT_CONTENT_TYPE.to_string(),
        };
        let mut current_file = File::open(&file_path.to_string())?;
        let mut content = vec![];
        current_file.read_to_end(&mut content)?;
        let content = oss::Bytes::from(content);

        println!("upload file {}", &object);
        match client
            .PutObject(&object)
            .with_forbid_overwrite(true)
            // .with_content_encoding(ContentEncoding::IDENTITY)
            // .with_cache_control(CacheControl::NoCache)
            // .with_content_disposition(ContentDisposition::ATTACHMENT(Some(
            //   "myfle.tmp".to_string(),
            // )))
            .with_content_type(&mime)
            // .with_content_language("zh-CN")
            .with_content(content)
            // .with_oss_meta("upload-at", Utc::now().timestamp().to_string().as_str())
            // .with_oss_meta("upload-by", "xtoss")
            // .with_encryption(ServerSideEncryption::AES256)
            // .with_expires(Utc::now())
            // .with_oss_tagging("tag1", "value1")
            // .with_oss_tagging("tag2", "value2")
            .execute()
            .await
        {
            Ok(Ok(_)) => (),
            Ok(Err(message)) => println!("oss error: {}", message.content()),
            Err(error) => println!("reqwest error: {}", error),
        }
    }

    Ok(())
}
