use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Part {
    #[serde(rename = "ETag")]
    pub etag: String,
    #[serde(rename = "PartNumber")]
    pub part_number: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CompleteMultipartUpload {
    #[serde(rename = "Part")]
    pub parts: Vec<Part>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CopyPartResult {
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub etag: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multipart_1() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<CopyPartResult>
  <LastModified>2019-04-09T07:01:56.000Z</LastModified>
  <ETag>"25A9F4ABFCC05743DF6E2C886C56****"</ETag>
</CopyPartResult>"#;

        let obj: CopyPartResult = quick_xml::de::from_str(&xml_content).unwrap();
        println!("{:#?}", &obj);
    }

    #[test]
    fn multipart_2() {
        let xml_content = r#"<CompleteMultipartUpload> 
<Part>
	<PartNumber>1</PartNumber>  
	<ETag>"3349DC700140D7F86A0784842780****"</ETag> 
</Part>  
<Part> 
<PartNumber>5</PartNumber>  
	<ETag>"8EFDA8BE206636A695359836FE0A****"</ETag> 
</Part>  
<Part> 
	<PartNumber>8</PartNumber>  
	<ETag>"8C315065167132444177411FDA14****"</ETag> 
</Part> 
</CompleteMultipartUpload>
"#;

        let obj: CompleteMultipartUpload = quick_xml::de::from_str(&xml_content).unwrap();
        println!("{:#?}", &obj);
    }

    #[test]
    fn multipart_3() {
        let xml_content = r#"<CompleteMultipartUploadResult xmlns=”http://doc.oss-cn-hangzhou.aliyuncs.com”>
	<EncodingType>url</EncodingType>
	<Location>http://oss-example.oss-cn-hangzhou.aliyuncs.com /multipart.data</Location>
	<Bucket>oss-example</Bucket>
	<Key>multipart.data</Key>
	<ETag>"B864DB6A936D376F9F8D3ED3BBE540****"</ETag>
</CompleteMultipartUploadResult>"#;

        let obj: CompleteMultipartUpload = quick_xml::de::from_str(&xml_content).unwrap();
        println!("{:#?}", &obj);
    }
}

