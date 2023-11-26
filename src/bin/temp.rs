use xt_oss::OssOptions;


fn main () {
	let mut options = OssOptions::from_env();
	options.access_key_id = "abcd".to_string();
	println!("{}", serde_json::to_string(&options).unwrap());
}