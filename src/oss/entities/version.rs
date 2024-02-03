use super::{bucket::Owner, StorageClass};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommonPrefixes {
  #[serde(rename = "Prefix")]
  pub prefix: String
}

/// 保存版本控制状态的容器
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VersioningStatus {
  /// 开启版本控制状态
  Enabled,
  /// 暂停版本控制状态
  Suspended,
}

impl fmt::Display for VersioningStatus {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Enabled => "Enabled",
        Self::Suspended => "Suspended",
      }
    )
  }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct VersioningConfiguration {
  #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
  pub status: Option<VersioningStatus>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DeleteMarker {
  #[serde(rename = "Key")]
  pub key: String,
  #[serde(rename = "VersionId")]
  pub version_id: String,
  #[serde(rename = "IsLatest")]
  pub is_latest: String,
  #[serde(rename = "LastModified")]
  pub last_modified: String,
  #[serde(rename = "Owner")]
  pub owner: Owner,
  // #[serde(rename = "Size")]
  // pub size: Option<u64>,
}

/*
 <DeleteMarker>
    <Key>tmp/test.text</Key>
    <VersionId>CAEQ2AEYgYDAo5qWtesYIiBmNjY4N2QyNjZlZjY0NjQ0YmNiMzM3YWQwNWQzN2Q2Yw--</VersionId>
    <IsLatest>true</IsLatest>
    <LastModified>2024-02-03T04:47:34.000Z</LastModified>
    <Owner>
      <ID>1508492296054765</ID>
      <DisplayName>1508492296054765</DisplayName>
    </Owner>
  </DeleteMarker>
*/

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Version {
  #[serde(rename = "Key")]
  pub key: String,
  #[serde(rename = "VersionId")]
  pub version_id: String,
  #[serde(rename = "IsLatest")]
  pub is_latest: bool,
  #[serde(rename = "LastModified")]
  pub last_modified: String,
  #[serde(rename = "ETag", skip_serializing_if = "Option::is_none")]
  pub etag: Option<String>,
  #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
  pub r#type: Option<String>,
  #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
  pub size: Option<u64>,
  #[serde(rename = "StorageClass", skip_serializing_if = "Option::is_none")]
  pub storage_class: Option<StorageClass>,
  #[serde(rename = "Owner")]
  pub owner: Owner,
  #[serde(rename = "RestoreInfo")]
  pub restore_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Item {
  Version(Version),
  DeleteMarker(DeleteMarker)
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListVersionsResult {
  #[serde(rename = "CommonPrefixes")]
  pub common_prefixes:Option<Vec<CommonPrefixes>>,
  #[serde(rename = "Delimiter")]
  pub delimiter: Option<String>,
  #[serde(rename = "EncodingType")]
  pub encoding_type: Option<String>,
  #[serde(rename = "IsTruncated")]
  pub is_truncated: bool,
  #[serde(rename = "KeyMarker")]
  pub key_marker: String,
  #[serde(rename = "VersionIdMarker")]
  pub version_id_marker:Option<String>,
  #[serde(rename = "NextKeyMarker")]
  pub next_key_marker: Option<String>,
  #[serde(rename = "NextVersionIdMarker")]
  pub next_version_id_marker: Option<String>,
  #[serde(rename = "MaxKeys")]
  pub max_keys: u64,
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "Owner")]
  pub owner: Option<Owner>,
  #[serde(rename = "Prefix")]
  pub prefix: String,
  #[serde(rename="$value", skip_serializing_if = "Option::is_none")]
  pub items: Option<Vec<Item>>
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn list_versions_result_1() {
    let xml_content = r#"<ListVersionsResult>
<Name>examplebucket-1250000000</Name>
<Prefix/>
<KeyMarker/>
<VersionIdMarker/>
<MaxKeys>1000</MaxKeys>
<IsTruncated>false</IsTruncated>
<Version>
    <Key>example-object-1.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-5T12:03:10.000Z</LastModified>
    <ETag>5B3C1A2E053D763E1B669CC607C5A0FE1****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner>
        <ID>1250000000</ID>
        <DisplayName>1250000000</DisplayName>
    </Owner>
</Version>
<Version>
    <Key>example-object-2.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-9T12:03:09.000Z</LastModified>
    <ETag>5B3C1A2E053D763E1B002CC607C5A0FE1****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner>
        <ID>1250000000</ID>
        <DisplayName>1250000000</DisplayName>
    </Owner>
</Version>
<Version>
    <Key>example-object-3.jpg</Key>
    <VersionId/>
    <IsLatest>true</IsLatest>
    <LastModified>2019-08-10T12:03:08.000Z</LastModified>
    <ETag>4B3F1A2E053D763E1B002CC607C5AGTRF****</ETag>
    <Size>20</Size>
    <StorageClass>Standard</StorageClass>
    <Owner>
        <ID>1250000000</ID>
        <DisplayName>1250000000</DisplayName>
    </Owner>
</Version>
</ListVersionsResult>"#;

    let object = quick_xml::de::from_str::<ListVersionsResult>(xml_content).unwrap();
    println!("{:#?}", object);
  }

  #[test]
  fn list_versions_result_2() {
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListVersionsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
    <Name>oss-example</Name>
    <Prefix></Prefix>
    <KeyMarker>example</KeyMarker>
    <VersionIdMarker>CAEQMxiBgICbof2D0BYiIGRhZjgwMzJiMjA3MjQ0ODE5MWYxZDYwMzJlZjU1****</VersionIdMarker>
    <MaxKeys>100</MaxKeys>
    <Delimiter></Delimiter>
    <IsTruncated>false</IsTruncated>
    <DeleteMarker>
        <Key>example</Key>
        <VersionId>CAEQMxiBgICAof2D0BYiIDJhMGE3N2M1YTI1NDQzOGY5NTkyNTI3MGYyMzJm****</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2019-04-09T07:27:28.000Z</LastModified>
        <Owner>
            <ID>1234512528586****</ID>
            <DisplayName>12345125285864390</DisplayName>
        </Owner>
    </DeleteMarker>
    <Version>
        <Key>example</Key>
        <VersionId>CAEQMxiBgMDNoP2D0BYiIDE3MWUxNzgxZDQxNTRiODI5OGYwZGMwNGY3MzZjN****</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2019-04-09T07:27:28.000Z</LastModified>
        <ETag>"250F8A0AE989679A22926A875F0A2****"</ETag>
        <Type>Normal</Type>
        <Size>93731</Size>
        <StorageClass>Standard</StorageClass>
        <Owner>
            <ID>1234512528586****</ID>
            <DisplayName>12345125285864390</DisplayName>
        </Owner>
    </Version>
    <Version>
        <Key>pic.jpg</Key>
        <VersionId>CAEQMxiBgMCZov2D0BYiIDY4MDllOTc2YmY5MjQxMzdiOGI3OTlhNTU0ODIx****</VersionId>
        <IsLatest>true</IsLatest>
        <LastModified>2019-04-09T07:27:28.000Z</LastModified>
        <ETag>"3663F7B0B9D3153F884C821E7CF4****"</ETag>
        <Type>Normal</Type>
        <Size>574768</Size>
        <StorageClass>Standard</StorageClass>
        <Owner>
            <ID>1234512528586****</ID>
            <DisplayName>12345125285864390</DisplayName>
        </Owner>
    </Version>
</ListVersionsResult>"#;

    let object: ListVersionsResult = quick_xml::de::from_str(xml_content).unwrap();

    println!("{:#?}", object);

  }

  #[test]
  fn list_versions_result_3() {
    let xml_content = r#"<ListVersionsResult xmlns="http://doc.oss-cn-hangzhou.aliyuncs.com">
<Name>oss-example</Name>
<Prefix></Prefix>
<KeyMarker>example</KeyMarker>
<VersionIdMarker>CAEQMxiBgICbof2D0BYiIGRhZjgwMzJiMjA3MjQ0ODE5MWYxZDYwMzJlZjU1****</VersionIdMarker>
<MaxKeys>100</MaxKeys>
<Delimiter></Delimiter>
<IsTruncated>false</IsTruncated>
<Version>
    <Key>exampleobject1.txt</Key>
    <VersionId>CAEQMxiBgICAof2D0BYiIDJhMGE3N2M1YTI1NDQzOGY5NTkyNTI3MGYyMzJm****</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <Owner>
        <ID>1234512528586****</ID>
        <DisplayName>12345125285864390</DisplayName>
    </Owner>
    </Version>
<Version>
    <Key>exampleobject2.txt</Key>
    <VersionId>CAEQMxiBgMDNoP2D0BYiIDE3MWUxNzgxZDQxNTRiODI5OGYwZGMwNGY3MzZjN****</VersionId>
    <IsLatest>false</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <ETag>"250F8A0AE989679A22926A875F0A2****"</ETag>
    <Type>Normal</Type>
    <Size>93731</Size>
    <StorageClass>Standard</StorageClass>
    <RestoreInfo>ongoing-request="true"</RestoreInfo>
    <Owner>
        <ID>1234512528586****</ID>
        <DisplayName>12345125285864390</DisplayName>
    </Owner>
    </Version>
<Version>
    <Key>exampleobject3.txt</Key>
    <VersionId>CAEQMxiBgMCZov2D0BYiIDY4MDllOTc2YmY5MjQxMzdiOGI3OTlhNTU0ODIx****</VersionId>
    <IsLatest>true</IsLatest>
    <LastModified>2019-04-09T07:27:28.000Z</LastModified>
    <ETag>"3663F7B0B9D3153F884C821E7CF4****"</ETag>
    <Type>Normal</Type>
    <Size>574768</Size>
    <StorageClass>Standard</StorageClass>
    <RestoreInfo>ongoing-request="false", expiry-date="Thr, 24 Mon 2020 12:40:33 GMT"</RestoreInfo>
    <Owner>
        <ID>1234512528586****</ID>
        <DisplayName>12345125285864390</DisplayName>
    </Owner>
    </Version>
</ListVersionsResult>"#;

    let object: ListVersionsResult = quick_xml::de::from_str(xml_content).unwrap();

    println!("{:#?}", object);
  }


  #[test]
  fn list_versions_result_4() {
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListVersionsResult>
  <Name>xuetube-dev</Name>
  <Prefix>course/video</Prefix>
  <KeyMarker></KeyMarker>
  <VersionIdMarker></VersionIdMarker>
  <MaxKeys>20</MaxKeys>
  <Delimiter>/</Delimiter>
  <IsTruncated>false</IsTruncated>
  <CommonPrefixes>
    <Prefix>course/video/</Prefix>
  </CommonPrefixes>
</ListVersionsResult>"#;

    let obj:ListVersionsResult = quick_xml::de::from_str(&xml_content).unwrap();
    println!("{:#?}", obj);

  }

  #[test]
  fn list_versions_result_5() {
    let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
    <ListVersionsResult>
      <Name>xtoss-ex1</Name>
      <Prefix></Prefix>
      <KeyMarker></KeyMarker>
      <VersionIdMarker></VersionIdMarker>
      <MaxKeys>100</MaxKeys>
      <Delimiter></Delimiter>
      <IsTruncated>false</IsTruncated>
      <DeleteMarker>
        <Key>tmp/test.text</Key>
        <VersionId>CAEQ2AEYgYDAo5qWtesYIiBmNjY4N2QyNjZlZjY0NjQ0YmNiMzM3YWQwNWQzN2Q2Yw--</VersionId>
        <IsLatest>true</IsLatest>
        <LastModified>2024-02-03T04:47:34.000Z</LastModified>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </DeleteMarker>
      <Version>
        <Key>tmp/test.text</Key>
        <VersionId>CAEQ2AEYgYDAs7WTtesYIiAxOTMxYmI3OTgxNzA0ZmM3YTNkZWE5NWYyODFhYWQxOQ--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T04:46:03.000Z</LastModified>
        <ETag>"C8B0FF27A844D2EECD81669DBAA544EB"</ETag>
        <Type>Normal</Type>
        <Size>12</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
      <DeleteMarker>
        <Key>tmp/test.txt</Key>
        <VersionId>CAEQ2AEYgYCAqMC8t.sYIiAyY2RkMWY1MTQ0NGU0ZDhjOGQ4NzFmN2JkMWM2NDk5OQ--</VersionId>
        <IsLatest>true</IsLatest>
        <LastModified>2024-02-03T07:28:18.000Z</LastModified>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </DeleteMarker>
      <Version>
        <Key>tmp/test.text</Key>
        <VersionId>CAEQ2AEYgYDAuu.RtesYIiA1NTAxODRkOGE2ODk0MTNkYWFkZDJhZTMyZjZlMDEzZg--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T04:45:12.000Z</LastModified>
        <ETag>"5EB63BBBE01EEED093CB22BB8F5ACDC3"</ETag>
        <Type>Normal</Type>
        <Size>11</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
      <Version>
        <Key>tmp/test.txt</Key>
        <VersionId>CAEQ2AEYgYCA1v6ot.sYIiBmZjU2NTQwOGEwZDc0MTMyYTU5ZjhlMmUyNGYwMjc3NA--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T07:17:39.000Z</LastModified>
        <ETag>"FC3FF98E8C6A0D3087D515C0473F8677"</ETag>
        <Type>Normal</Type>
        <Size>12</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
      <Version>
        <Key>tmp/test.txt</Key>
        <VersionId>CAEQ2AEYgYCAur2ot.sYIiBmM2M5MDBjNDE0OWE0OGVmYTYwN2Q1OWIyMGNlZDQ3Ng--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T07:17:22.000Z</LastModified>
        <ETag>"5EB63BBBE01EEED093CB22BB8F5ACDC3"</ETag>
        <Type>Normal</Type>
        <Size>11</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
      <Version>
        <Key>tmp/test.txt</Key>
        <VersionId>CAEQ2AEYgYDA_66nt.sYIiA5NTUzMjE0YzcwZGE0N2MyYTUxY2QxNmY1MGIxNjgzMQ--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T07:16:45.000Z</LastModified>
        <ETag>"5EB63BBBE01EEED093CB22BB8F5ACDC3"</ETag>
        <Type>Normal</Type>
        <Size>11</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
      <Version>
        <Key>tmp/test.txt</Key>
        <VersionId>CAEQ2AEYgYDA0eOXtesYIiA3MDc1YzU3MjUwYmI0NTYwYjM2NmI4YmY5NWNjOWQxYQ--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T04:48:26.000Z</LastModified>
        <ETag>"5EB63BBBE01EEED093CB22BB8F5ACDC3"</ETag>
        <Type>Normal</Type>
        <Size>11</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
      <Version>
        <Key>tmp/test.txt</Key>
        <VersionId>CAEQ2AEYgYDA4N2WtesYIiA4MDQ2ZjQzNDZmZTA0MjU1OTA3MjFlNjNiNmFhNDE5Yw--</VersionId>
        <IsLatest>false</IsLatest>
        <LastModified>2024-02-03T04:47:52.000Z</LastModified>
        <ETag>"C8B0FF27A844D2EECD81669DBAA544EB"</ETag>
        <Type>Normal</Type>
        <Size>12</Size>
        <StorageClass>Archive</StorageClass>
        <Owner>
          <ID>1508492296054765</ID>
          <DisplayName>1508492296054765</DisplayName>
        </Owner>
      </Version>
    </ListVersionsResult>"#;
    let obj:ListVersionsResult = quick_xml::de::from_str(&xml_content).unwrap();
    println!("{:#?}", obj);
  }

}
