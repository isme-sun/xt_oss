use serde::{Deserialize, Serialize};

#[derive(Debug, Clone,Serialize, Deserialize, Default)]
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
    )]
    pub create_time: Option<String>,
    #[serde(
        rename = "LastModifyTime",
        skip_serializing_if = "Option::is_none",
    )]
    pub last_modify_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StyleList {
    #[serde(rename = "Style")]
    pub style: Option<Vec<Style>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn style_1() {
        let xml_content = r#"<Style>
  <Name>imagestyle</Name>
  <Content>image/resize,p_50</Content>
  <Category>image</Category>
  <CreateTime>Wed, 20 May 2020 12:07:15 GMT</CreateTime>
  <LastModifyTime>Wed, 20 May 2020 12:07:15 GMT</LastModifyTime>
</Style>"#;

        let style: Style = quick_xml::de::from_str(&xml_content).unwrap();
        assert_eq!(style.category, Some("image".to_string()));
    }

    #[test]
    fn style_2() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<StyleList>
    <Style>
        <Name>imagestyle</Name>
        <Content>image/resize,p_50</Content>
        <Category>image</Category>
        <CreateTime>Wed, 20 May 2020 12:07:15 GMT</CreateTime>
        <LastModifyTime>Wed, 20 May 2020 12:07:15 GMT</LastModifyTime>
    </Style>
    <Style>
        <Name>imagestyle1</Name>
        <Content>image/resize,w_200</Content>
        <Category>image</Category>
        <CreateTime>Wed, 20 May 2020 12:08:04 GMT</CreateTime>
        <LastModifyTime>Wed, 20 May 2020 12:07:15 GMT</LastModifyTime>
    </Style>
    <Style>
        <Name>imagestyle2</Name>
        <Content>image/resize,w_300</Content>
        <Category>image</Category>
        <CreateTime>Fri, 12 Mar 2021 06:19:13 GMT</CreateTime>
        <LastModifyTime>Wed, 20 May 2020 12:07:15 GMT</LastModifyTime>
    </Style>
</StyleList>"#;

        let style_list: StyleList = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "image/resize,p_50";
        let right = &style_list.style.unwrap()[0].content;
        assert_eq!(left, right);
    }

    #[test]
    fn style_3() {
        let style = Style {
            name: "imagestyle".to_string(),
            content: "image/resize,p_50".to_string(),
            category: Some("image".to_string()),
            create_time: None,
            last_modify_time: None,
        };

        let left = quick_xml::se::to_string(&style).unwrap();
        let right = r#"<Style><Name>imagestyle</Name><Content>image/resize,p_50</Content><Category>image</Category></Style>"#;
        assert_eq!(left, right);
    }
}
