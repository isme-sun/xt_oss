use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoggingEnabled {
	#[serde(rename = "TargetBucket", skip_serializing_if = "Option::is_none")]
	pub target_bucket: Option<String>,
	#[serde(rename = "TargetPrefix", skip_serializing_if = "Option::is_none")]
	pub target_prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BucketLoggingStatus {
	#[serde(rename = "LoggingEnabled", skip_serializing_if = "Option::is_none")]
	pub logging_enabled: Option<LoggingEnabled>,
}

#[cfg(test)]
pub mod tests {
	use super::*;

	#[test]
	fn bucket_logging_status() {
		let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<BucketLoggingStatus>
	<LoggingEnabled>
		<TargetBucket>examplebucket</TargetBucket>
		<TargetPrefix>MyLog-</TargetPrefix>
	</LoggingEnabled>
</BucketLoggingStatus>"#;
		let obj = quick_xml::de::from_str::<BucketLoggingStatus>(&xml).unwrap();
		let left = "MyLog-";
		let right = obj.logging_enabled.unwrap().target_prefix.unwrap();
		assert_eq!(left, right);
	}
}
