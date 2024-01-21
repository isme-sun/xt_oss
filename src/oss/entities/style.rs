use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Style {
	#[serde(rename = "Name")]
	pub name: String,
	#[serde(rename = "Content")]
	pub content: String,
	#[serde(rename = "Category")]
	pub category: Option<String>,
	#[serde(
		rename = "CreateTime",
		skip_serializing_if = "Option::is_none",
		with = "super::private::serde_date::gmt_option"
	)]
	pub create_time: Option<DateTime<Utc>>,
	#[serde(
		rename = "LastModifyTime",
		skip_serializing_if = "Option::is_none",
		with = "super::private::serde_date::gmt_option"
	)]
	pub last_modify_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StyleList {
	#[serde(rename = "Style")]
	pub style: Vec<Style>,
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn style() {
		let xml_origin = r#"<Style><Name>imagestyle</Name><Content>image/resize,p_50</Content><Category>image</Category><CreateTime>Wed, 20 May 2020 12:07:15 GMT</CreateTime><LastModifyTime>Wed, 20 May 2020 12:07:15 GMT</LastModifyTime></Style>"#;

		let style = Style {
			name: "imagestyle".to_string(),
			content: "image/resize,p_50".to_string(),
			category: Some("image".to_string()),
			create_time: None,
			last_modify_time: None,
		};

		let xml_gen = quick_xml::se::to_string(&style).unwrap();
		println!("{}", xml_gen);

		let style1 = quick_xml::de::from_str::<Style>(&xml_origin).unwrap();
		println!("{:#?}", style1);
	}
}
