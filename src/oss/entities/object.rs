use std::fmt;

use serde::{Deserialize, Serialize};

pub mod delete_multiple {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Delete {
        #[serde(rename = "Quiet")]
        pub quiet: Option<bool>,
        #[serde(rename = "Object")]
        pub object: Vec<Object>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Object {
        #[serde(rename = "Key")]
        pub key: String,
        #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
        pub version_id: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct DeleteResult {
        #[serde(rename = "Deleted")]
        pub deleted: Vec<Deleted>,
        #[serde(rename = "EncodingType")]
        pub encoding_type: Option<String>,
    }

    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct Deleted {
        #[serde(rename = "Key")]
        pub key: String,
        #[serde(rename = "VersionId")]
        pub version_id: Option<String>,
        #[serde(rename = "DeleteMarker")]
        pub delete_marker: Option<String>,
        #[serde(rename = "DeleteMarkerVersionId")]
        pub delete_marker_version_id: Option<String>,
    }
}

#[derive(Debug,Clone, Default, Serialize, Deserialize)]
pub struct Object {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub etag: String,
    #[serde(rename = "Size")]
    pub size: i32,
    #[serde(rename = "StorageClass")]
    pub storage_class: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum Tier {
    Expedited,
    #[default]
    Standard,
    Bulk,
}

impl fmt::Display for Tier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Expedited => "Expedited",
            Self::Standard => "Standard",
            Self::Bulk => "Bulk",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CopyObjectResult {
    #[serde(rename = "etag")]
    pub etag: Option<String>,
    #[serde(rename = "LastModified")]
    pub last_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JobParameters {
    #[serde(rename = "Tier")]
    pub tier: Tier,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RestoreRequest {
    #[serde(rename(deserialize = "Days"))]
    pub days: u8,
    #[serde(rename = "JobParameters", skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<JobParameters>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum MetadataDirective {
    #[default]
    COPY,
    REPLACE,
}

impl fmt::Display for MetadataDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::COPY => "COPY",
            Self::REPLACE => "REPLACE",
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone,Serialize, Deserialize, Default)]
pub enum TaggingDirective {
    #[default]
    COPY,
    REPLACE,
}

impl fmt::Display for TaggingDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::COPY => "COPY",
            Self::REPLACE => "REPLACE",
        };
        write!(f, "{}", value)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::oss::entities::object::RestoreRequest;

    use super::{
        delete_multiple::{Delete, DeleteResult},
        JobParameters, Tier,
    };

    #[test]
    fn restore_request_1() {
        let xml_content = r#"<RestoreRequest>
  <Days>2</Days>
  <JobParameters>
    <Tier>Standard</Tier>
  </JobParameters>
</RestoreRequest>"#;

        let entry: RestoreRequest = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "Standard";
        let right = entry.job_parameters.unwrap().tier.to_string();
        assert_eq!(left, right);
    }

    #[test]
    fn restore_request_2() {
        let restore = RestoreRequest {
            days: 7,
            job_parameters: Some(JobParameters {
                tier: Tier::Expedited,
            }),
        };
        let left =
            "<RestoreRequest><days>7</days><JobParameters><Tier>Expedited</Tier></JobParameters></RestoreRequest>";
        let right = quick_xml::se::to_string(&restore).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn restore_request_3() {
        let restore = RestoreRequest {
            days: 7,
            job_parameters: None,
        };
        let left = "<RestoreRequest><days>7</days></RestoreRequest>";
        let right = quick_xml::se::to_string(&restore).unwrap();
        assert_eq!(left, right);
    }

    #[test]
    fn deleted_1() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<Delete> 
  <Quiet>false</Quiet>  
  <Object> 
    <Key>multipart.data</Key> 
  </Object>  
  <Object> 
    <Key>test.jpg</Key> 
  </Object>  
  <Object> 
    <Key>demo.jpg</Key> 
  </Object> 
</Delete>"#;
        let obj: Delete = quick_xml::de::from_str(&xml_content).unwrap();
        let left = &obj.object[0].key;
        let right = "multipart.data";
        assert_eq!(left, right);
    }

    #[test]
    fn delete_result_1() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
  <Deleted>
    <Key>multipart.data</Key>
  </Deleted>
  <Deleted>
    <Key>test.jpg</Key>
  </Deleted>
  <Deleted>
    <Key>demo.jpg</Key>
  </Deleted>
</DeleteResult>"#;
        let obj: DeleteResult = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "multipart.data";
        let right = &obj.deleted[0].key;
        assert_eq!(left, right);
    }

    #[test]
    fn delete_result_2() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
  <Deleted>
    <Key>multipart.data</Key>
    <VersionId>CAEQNRiBgIDyz.6C0BYiIGQ2NWEwNmVhNTA3ZTQ3MzM5ODliYjM1ZTdjYjA4****</VersionId>
  </Deleted>
</DeleteResult>"#;
        let obj: DeleteResult = quick_xml::de::from_str(&xml_content).unwrap();
        assert_eq!("multipart.data", obj.deleted[0].key);
    }

    #[test]
    fn delete_result_3() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
  <Deleted>
    <Key>demo.jpg</Key>
    <VersionId>CAEQNRiBgICEoPiC0BYiIGMxZWJmYmMzYjE0OTQ0ZmZhYjgzNzkzYjc2NjZk****</VersionId>
    <DeleteMarker>true</DeleteMarker>
    <DeleteMarkerVersionId>THUQNRiBgICEoPiC0BYiIGMxZWJmYmMzYjE0OTQ0ZmZhYjgzNzkzYjc2NjZk****</DeleteMarkerVersionId>
  </Deleted>
</DeleteResult>"#;

        let obj: DeleteResult = quick_xml::de::from_str(&xml_content).unwrap();
        assert_eq!("demo.jpg", obj.deleted[0].key);
    }

    #[test]
    fn delete_result_4() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<DeleteResult>
  <Deleted>
    <Key>multipart.data</Key>
    <DeleteMarker>true</DeleteMarker>
    <DeleteMarkerVersionId>CAEQMhiBgIDXiaaB0BYiIGQzYmRkZGUxMTM1ZDRjOTZhNjk4YjRjMTAyZjhl****</DeleteMarkerVersionId>
  </Deleted>
  <Deleted>
    <Key>test.jpg</Key>
      <DeleteMarker>true</DeleteMarker>
      <DeleteMarkerVersionId>CAEQMhiBgIDB3aWB0BYiIGUzYTA3YzliMzVmNzRkZGM5NjllYTVlMjYyYWEy****</DeleteMarkerVersionId>
  </Deleted>
</DeleteResult>"#;

        let obj: DeleteResult = quick_xml::de::from_str(&xml_content).unwrap();
        assert_eq!("multipart.data", obj.deleted[0].key);
    }
}
