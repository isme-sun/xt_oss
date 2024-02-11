use std::collections::HashMap;

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

        println!("{}", &tagging.to_query());
    }

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

        let tagging = Tagging { tag_set };
        let content = serde_qs::to_string(&tagging).unwrap();
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
        println!("{}", &c.to_query());
    }
}
