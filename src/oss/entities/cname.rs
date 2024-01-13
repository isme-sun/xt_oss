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
pub struct CnameToken {
    #[serde(rename = "Bucket")]
    pub bucket: String,
    #[serde(rename = "Cname")]
    pub cname: String,
    #[serde(rename = "Token")]
    pub token: String,
    #[serde(rename = "ExpireTime", with = "super::private::serde_date::gmt")]
    pub expire_time: DateTime<Utc>,
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

pub mod builder {

    use super::BucketCnameConfiguration;

    #[derive(Debug, Default)]
    pub struct BucketCnameConfigurationBuilder {
        pub bucket_cname_configuration: BucketCnameConfiguration,
    }

    impl BucketCnameConfigurationBuilder {
        pub fn new() -> Self {
            BucketCnameConfigurationBuilder {
                bucket_cname_configuration: BucketCnameConfiguration::default(),
            }
        }

        pub fn with_domain(mut self, value: &str) -> Self {
            self.bucket_cname_configuration.cname.domain = value.to_string();
            self
        }

        // pub fn with_cert_id(mut self, value: &str) -> Self {
        //     let certificate =
        //         if let Some(mut certificate) = self.bucket_cname_configuration.cname.certificate {
        //             certificate.cert_id = value.to_string();
        //             certificate
        //         } else {
        //             let mut certificate = super::Certificate::default();
        //             certificate.cert_id = value.to_string();
        //             certificate
        //         };
        //     self.bucket_cname_configuration.cname.certificate = Some(certificate);
        //     self
        // }

        pub fn config(&self) -> String {
            quick_xml::se::to_string(&self.bucket_cname_configuration).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::builder::BucketCnameConfigurationBuilder;
    use crate::oss::{
        entities::cname::{CnameToken, ListCnameResult},
        GMT_DATE_FMT,
    };

    #[test]
    fn cname_token() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<CnameToken>
	<Bucket>examplebucket</Bucket>
	<Cname>example.com</Cname>;
	<Token>be1d49d863dea9ffeff3df7d6455****</Token>
	<ExpireTime>Wed, 23 Feb 2022 21:16:37 GMT</ExpireTime>
</CnameToken>"#;
        let obj = quick_xml::de::from_str::<CnameToken>(&xml).unwrap();
        let left = "Wed, 23 Feb 2022 21:16:37 GMT";
        let right = obj.expire_time.format(GMT_DATE_FMT).to_string();
        assert_eq!(left, right);
    }

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

    #[test]
    fn bucket_cname_configuration_builder() {
        let builder =
            BucketCnameConfigurationBuilder::default().with_domain("https://dev.xuetube.com");

        print!("{}", builder.config());
    }
}
