use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
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

impl Tagging {
    pub fn to_query(&self) -> String {
        let temp: HashMap<_, _> = self
            .tag_set
            .tag
            .as_ref()
            .unwrap_or(&vec![])
            .iter()
            .map(|tag| (tag.key.clone(), tag.value.clone()))
            .collect();

        serde_qs::to_string(&temp).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn tagging_to_query() {
        let tag = Tag {
            key: "name".to_string(),
            value: "xtoss".to_string(),
        };
        let tag1 = Tag {
            key: "version".to_string(),
            value: "测试".to_string(),
        };

        let tag_set = TagSet {
            tag: Some(vec![tag, tag1]),
        };
        let tagging = Tagging { tag_set };
        let query = tagging.to_query();
        assert!(query.contains("xtoss"));
    }

    #[test]
    fn tagging() {
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

        let content: Tagging = quick_xml::de::from_str(xml).unwrap();
        assert_eq!("key1", content.tag_set.tag.unwrap()[0].key);

    }
}
