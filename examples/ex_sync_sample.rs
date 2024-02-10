use std::{env, fs::File, io::Read};
use walkdir::{DirEntry, WalkDir};
use xt_oss::{oss, utils};

fn only_file(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map_or(false, |s| entry.depth() == 0 || !s.starts_with("."))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv().ok();
  let option = utils::options_from_env();
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

    match client
      .PutObject(&object)
      .with_forbid_overwrite(false)
      .with_content_type(&mime)
      .with_content(content)
      .execute()
      .await
    {
      Ok(Ok(_)) => println!("upload file: {}", &object),
      Ok(Err(message)) => println!("oss error: {}", message.content()),
      Err(error) => println!("reqwest error: {}", error),
    }
  }

  Ok(())
}
