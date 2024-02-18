use std::fmt;

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct JobParameters {
    #[serde(rename = "Tier")]
    pub tier: Tier,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RestoreRequest {
    #[serde(rename(deserialize = "Days"))]
    pub days: u8,
    #[serde(rename = "JobParameters", skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<JobParameters>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
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

    use super::{JobParameters, Tier};

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
            job_parameters: Some(JobParameters { tier: Tier::Expedited }),
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
}
