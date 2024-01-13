use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
// -------------------------------------------------------
// bcuked_cname                                         //
// -------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListCnameResult {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Owner")]
    pub owner: String,
    #[serde(rename = "Cname")]
    pub cname: Option<Vec<Cname>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Certificate {
    #[serde(rename(deserialize = "Type"))]
    pub r#type: String,
    #[serde(rename(deserialize = "CertId"))]
    pub cert_id: String,
    #[serde(rename(deserialize = "Status"))]
    pub status: String,
    #[serde(rename(deserialize = "CreationDate"))]
    pub creation_date: String,
    #[serde(rename(deserialize = "Fingerprint"))]
    pub fingerprint: String,
    #[serde(
        rename(deserialize = "ValidStartDate"),
        with = "super::private::serde_date::gmt"
    )]
    pub valid_start_date: DateTime<Utc>,
    #[serde(
        rename(deserialize = "ValidEndDate",),
        with = "super::private::serde_date::gmt"
    )]
    pub valid_end_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Cname {
    #[serde(rename = "Domain")]
    pub domain: String,
    #[serde(
        rename = "LastModified",
        with = "super::private::serde_date::utc_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_modified: Option<DateTime<Utc>>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Certificate", skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
    #[serde(rename = "Certificate", skip_serializing_if = "Option::is_none")]
    pub certificate_configuration: Option<CertificateConfiguration>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct BucketCnameConfiguration {
    pub cname: Cname,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct CertificateConfiguration {
    #[serde(rename = "CertId")]
    pub cert_id: String,
    #[serde(rename = "Certificate")]
    pub certificate: String,
    #[serde(rename = "PrivateKey")]
    pub private_key: String,
    #[serde(rename = "PreviousCertId")]
    pub previous_cert_id: String,
    #[serde(rename = "ForceDeleteCertificate")]
    pub force_delete_certificate: String,
}

#[cfg(test)]
mod tests {
    use crate::oss::entities::cname::ListCnameResult;

    #[test]
    fn list_cname_result() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<ListCnameResult>
	<Bucket>targetbucket</Bucket>
	<Owner>testowner</Owner>
	<Cname>
		<Domain>example.com</Domain>
		<LastModified>2021-09-15T02:35:07.000Z</LastModified>
		<Status>Enabled</Status>
		<Certificate>
			<Type>CAS</Type>
			<CertId>493****-cn-hangzhou</CertId>
			<Status>Enabled</Status>
			<CreationDate>Wed, 15 Sep 2021 02:35:06 GMT</CreationDate>
			<Fingerprint>DE:01:CF:EC:7C:A7:98:CB:D8:6E:FB:1D:97:EB:A9:64:1D:4E:**:**</Fingerprint>
			<ValidStartDate>Wed, 12 Apr 2023 10:14:51 GMT</ValidStartDate>
			<ValidEndDate>Mon, 4 May 2048 10:14:51 GMT</ValidEndDate>
		</Certificate>
	</Cname>
	<Cname>
		<Domain>example.org</Domain>
		<LastModified>2021-09-15T02:34:58.000Z</LastModified>
		<Status>Enabled</Status>
	</Cname>
	<Cname>
		<Domain>example.edu</Domain>
		<LastModified>2021-09-15T02:50:34.000Z</LastModified>
		<Status>Enabled</Status>
	</Cname>
</ListCnameResult>"#;
        let obj: ListCnameResult = quick_xml::de::from_str(&xml).unwrap();
        let cname = obj.cname.unwrap()[0].clone();
        let cert = cname.certificate.unwrap();
        assert_eq!("CAS", cert.r#type);
    }
}
