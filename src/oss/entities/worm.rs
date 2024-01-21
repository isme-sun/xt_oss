use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WormConfiguration {
	#[serde(rename = "WormId")]
	pub worm_id: String,
	#[serde(rename = "State")]
	pub state: String,
	#[serde(rename = "RetentionPeriodInDays")]
	pub retention_period_in_days: i32,
	#[serde(rename = "CreationDate")]
	pub creation_date: String,
}
