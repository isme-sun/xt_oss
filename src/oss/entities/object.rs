use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    #[serde(rename(deserialize = "Key"))]
    pub key: String,
    #[serde(rename(deserialize = "LastModified"))]
    pub last_modified: String,
    #[serde(rename(deserialize = "ETag"))]
    pub etag: String,
    #[serde(rename(deserialize = "Size"))]
    pub size: i32,
    #[serde(rename(deserialize = "StorageClass"))]
    pub storage_class: String,
}
