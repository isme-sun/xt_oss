use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
	#[serde(rename = "Key")]
	pub key: String,
	#[serde(rename = "Value")]
	pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TagSet {
	#[serde(rename = "Tag")]
	pub(crate) tag: Option<Vec<Tag>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Tagging {
	#[serde(rename = "TagSet")]
	pub tag_set: TagSet,
}
#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn tagging() {
		let tag = Tag {
			key: "key1".to_string(),
			value: "value1".to_string(),
		};
		let tag1 = Tag {
			key: "key1".to_string(),
			value: "value1".to_string(),
		};

		let tag_set = TagSet {
			tag: Some(vec![tag, tag1]),
		};

		let tag_sets = Tagging { tag_set };
		let content = quick_xml::se::to_string(&tag_sets).unwrap();
		println!("{}", content);

		let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Tagging>
	<TagSet>
		<Tag>
			<Key>key1</Key>
			<Value>value1</Value>
		</Tag>
		<Tag>
			<Key>key2</Key>
			<Value>value2</Value>
		</Tag>
	</TagSet>
</Tagging>"#;

		let c: Tagging = quick_xml::de::from_str(xml).unwrap();
		println!("{:#?}", c);
	}
}
