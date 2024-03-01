use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum Payer {
		#[default]
    BucketOwner,
    Requester,
}

impl fmt::Display for Payer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BucketOwner => write!(f, "{}", "BucketOwner"),
            Self::Requester => write!(f, "{}", "Requester"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RequestPaymentConfiguration {
		#[serde(rename="Payer")]
    pub payer: Payer
}

#[cfg(test)]
pub mod tests {
    use super::RequestPaymentConfiguration;

	#[test]
	fn request_payment_configuration() {
		let xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<RequestPaymentConfiguration>
	<Payer>BucketOwner</Payer>
</RequestPaymentConfiguration>"#;
		let obj:RequestPaymentConfiguration = quick_xml::de::from_str(&xml_content).unwrap();
		assert_eq!("BucketOwner", &obj.payer.to_string());
	}

}