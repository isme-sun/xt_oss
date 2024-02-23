use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct InitiateMultipartUploadResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Part {
    #[serde(rename = "PartNumber")]
    pub part_number: u64,
    #[serde(rename = "LastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "ETag")]
    pub etag: String,
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListPartsResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    #[serde(rename = "NextPartNumberMarker")]
    pub next_part_number_marker: String,
    #[serde(rename = "MaxParts")]
    pub max_parts: u64,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: bool,
    #[serde(rename = "Part", skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ListMultipartUploadsResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "KeyMarker")]
    pub key_marker: String,
    #[serde(rename = "UploadIdMarker")]
    pub upload_id_marker: String,
    #[serde(rename = "NextKeyMarker")]
    pub next_key_marker: String,
    #[serde(rename = "NextUploadIdMarker")]
    pub next_upload_id_marker: String,
    #[serde(rename = "Delimiter")]
    pub delimiter: String,
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "MaxUploads")]
    pub max_uploads: String,
    #[serde(rename = "IsTruncated")]
    pub is_truncated: String,
    #[serde(rename = "Upload")]
    pub uploads: Option<Vec<Upload>>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Upload {
    #[serde(rename = "Key")]
    pub key: String,
    #[serde(rename = "UploadId")]
    pub upload_id: String,
    #[serde(rename = "Initiated")]
    pub initiated: String,
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CompleteMultipartUploadResult {
    #[serde(rename = "EncodingType", skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "Location")]
    pub location: String,
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Key")]
    pub key: String,
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
        let left = "2019-04-09T07:01:56.000Z";
        let right = obj.last_modified;
        assert_eq!(left, right);
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
        let left = 1u64;
        let right = obj.parts[0].part_number;
        assert_eq!(left, right);
    }

    #[test]
    fn multipart_3() {
        let xml_content = r#"<CompleteMultipartUploadResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
<EncodingType>url</EncodingType>
  <Location>http://oss-example.oss-cn-hangzhou.aliyuncs.com /multipart.data</Location>
  <Bucket>oss-example</Bucket>
  <Key>multipart.data</Key>
  <ETag>"B864DB6A936D376F9F8D3ED3BBE540****"</ETag>
</CompleteMultipartUploadResult>"#;

        let obj: CompleteMultipartUploadResult = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "multipart.data";
        let right = &obj.key;
        assert_eq!(left, right);
    }

    #[test]
    fn multipart_4() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<CompleteMultipartUploadResult>
  <Location>http://oss-example.oss-cn-hangzhou.aliyuncs.com/multipart.data</Location>
  <Bucket>oss-example</Bucket>
  <Key>multipart.data</Key>
  <ETag>"097DE458AD02B5F89F9D0530231876****"</ETag>
</CompleteMultipartUploadResult>
"#;

        let obj: CompleteMultipartUploadResult = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "multipart.data";
        let right = &obj.key;
        assert_eq!(left, right);
    }

    #[test]
    fn multipart_5() {
        let xml_content = r#"<ListMultipartUploadsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
  <Bucket>oss-example</Bucket>
  <KeyMarker></KeyMarker>
  <UploadIdMarker></UploadIdMarker>
  <NextKeyMarker>oss.avi</NextKeyMarker>
  <NextUploadIdMarker>0004B99B8E707874FC2D692FA5D77D3F</NextUploadIdMarker>
  <Delimiter></Delimiter>
  <Prefix></Prefix>
  <MaxUploads>1000</MaxUploads>
  <IsTruncated>false</IsTruncated>
  <Upload>
    <Key>multipart.data</Key>
    <UploadId>0004B999EF518A1FE585B0C9360DC4C8</UploadId>
    <Initiated>2012-02-23T04:18:23.000Z</Initiated>
  </Upload>
  <Upload>
    <Key>multipart.data</Key>
    <UploadId>0004B999EF5A239BB9138C6227D6****</UploadId>
    <Initiated>2012-02-23T04:18:23.000Z</Initiated>
  </Upload>
  <Upload>
    <Key>oss.avi</Key>
    <UploadId>0004B99B8E707874FC2D692FA5D7****</UploadId>
    <Initiated>2012-02-23T06:14:27.000Z</Initiated>
  </Upload>
</ListMultipartUploadsResult>
"#;

        let obj: ListMultipartUploadsResult = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "multipart.data";
        let right = &obj.uploads.unwrap()[0].key;
        assert_eq!(left, right);
    }

    #[test]
    fn multipart_6() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListPartsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
  <Bucket>multipart_upload</Bucket>
  <Key>multipart.data</Key>
  <UploadId>0004B999EF5A239BB9138C6227D6****</UploadId>
  <NextPartNumberMarker>5</NextPartNumberMarker>
  <MaxParts>1000</MaxParts>
  <IsTruncated>false</IsTruncated>
  <Part>
     <PartNumber>1</PartNumber>
     <LastModified>2012-02-23T07:01:34.000Z</LastModified>
     <ETag>"3349DC700140D7F86A0784842780****"</ETag>
     <Size>6291456</Size>
  </Part>
  <Part>
     <PartNumber>2</PartNumber>
     <LastModified>2012-02-23T07:01:12.000Z</LastModified>
     <ETag>"3349DC700140D7F86A0784842780****"</ETag>
     <Size>6291456</Size>
  </Part>
  <Part>
     <PartNumber>5</PartNumber>
     <LastModified>2012-02-23T07:02:03.000Z</LastModified>
     <ETag>"7265F4D211B56873A381D321F586****"</ETag>
     <Size>1024</Size>
  </Part>
</ListPartsResult>
"#;
        let obj: ListPartsResult = quick_xml::de::from_str(&xml_content).unwrap();
        let left = r#""3349DC700140D7F86A0784842780****""#;
        let right = &obj.parts.unwrap()[0].etag;
        assert_eq!(left, right);
    }

    #[test]
    fn multipart_7() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<InitiateMultipartUploadResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
  <Bucket>oss-example</Bucket>
  <Key>multipart.data</Key>
  <UploadId>0004B9894A22E5B1888A1E29F823****</UploadId>
</InitiateMultipartUploadResult>"#;
        let obj: InitiateMultipartUploadResult = quick_xml::de::from_str(&xml_content).unwrap();
        let left = "oss-example";
        let right = obj.bucket;
        assert_eq!(left, right);
    }

    #[test]
    fn multipart_8() {
        let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListPartsResult>
  <Bucket>xtoss-ex1</Bucket>
  <Key>tmp/temp.jpg</Key>
  <UploadId>149E85A3897241A2B8A5F5BBFADA5D88</UploadId>
  <StorageClass>Standard</StorageClass>
  <PartNumberMarker>0</PartNumberMarker>
  <NextPartNumberMarker></NextPartNumberMarker>
  <MaxParts>1000</MaxParts>
  <IsTruncated>false</IsTruncated>
</ListPartsResult>"#;
        let obj: ListPartsResult = quick_xml::de::from_str(&xml_content).unwrap();
        assert!(obj.parts.is_none());
    }
}
