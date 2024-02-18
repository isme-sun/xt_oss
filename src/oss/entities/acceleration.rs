use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferAccelerationConfiguration {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn transfer_acceleration_configuration() {
        let xml = r#"<TransferAccelerationConfiguration>
  <Enabled>true</Enabled>
</TransferAccelerationConfiguration>"#;
        let object1: TransferAccelerationConfiguration = quick_xml::de::from_str(xml).unwrap();

        let object2 = TransferAccelerationConfiguration { enabled: true };

        assert_eq!(object1.enabled, object2.enabled)
    }
}
