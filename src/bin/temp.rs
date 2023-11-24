use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
struct OSSObject {
		#[serde(rename(deserialize = "Key"))]
		pub key:String,
		#[serde(rename(deserialize = "LastModified"))]
		pub last_modified:String,
		#[serde(rename(deserialize = "ETag"))]
		pub etag: String,
		#[serde(rename(deserialize = "Size"))]
		pub size: i32,
		#[serde(rename(deserialize = "StorageClass"))]
		pub storage_class: String
}

#[allow(non_snake_case)]
#[derive(Debug,Serialize, Deserialize)]
struct ListBucketResult {
	#[serde(rename(deserialize = "Name"))]
	pub name: String,
	#[serde(rename(deserialize = "Prefix"))]
	pub prefix: String,
	#[serde(rename(deserialize = "MaxKeys"))]
	pub max_keys: i32,
	#[serde(rename(deserialize = "EncodingType"))]
	pub encoding_type: String,
	#[serde(rename(deserialize = "IsTruncated"))]
	pub is_truncated: bool,
	#[serde(rename(deserialize = "KeyCount"))]
	pub key_count: i32,
	// #[serde(rename = "$value")]
	#[serde(rename(deserialize = "Contents"))]
	pub contents: Vec<OSSObject>,
}


fn main() {
	let file = "./assets/list_object_v2.xml";
	let content = fs::read_to_string(file).unwrap();
	let value:ListBucketResult = serde_xml_rs::from_str(&content).unwrap();
	println!("{}", serde_json::to_string(&value).unwrap());
}