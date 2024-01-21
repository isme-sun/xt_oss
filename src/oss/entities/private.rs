// ! todo ?? 这样可以不？
pub(crate) mod serde_date {

	pub(crate) mod utc {
		use chrono::{DateTime, Utc};
		use serde::{self, Deserialize, Deserializer, Serializer};

		const FORMAT: &str = "%Y-%m-%dT%H:%M:%S.000Z";

		pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let s = format!("{}", date.format(FORMAT));
			serializer.serialize_str(&s)
		}

		pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
		where
			D: Deserializer<'de>,
		{
			let s = String::deserialize(deserializer)?;
			let dt = s.parse::<DateTime<Utc>>().unwrap();
			Ok(dt)
		}
	}

	pub(crate) mod utc_option {
		use chrono::{DateTime, Utc};
		use serde::{self, Deserialize, Deserializer, Serializer};

		const FORMAT: &str = "%Y-%m-%dT%H:%M:%S.000Z";

		pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match date {
				Some(date) => {
					let s = format!("{}", date.format(FORMAT));
					serializer.serialize_str(&s)
				}
				None => serializer.serialize_str("null"),
			}
		}

		pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
		where
			D: Deserializer<'de>,
		{
			let s = String::deserialize(deserializer)?;
			let dt = s.parse::<DateTime<Utc>>().unwrap();
			Ok(Some(dt))
		}
	}

	pub(crate) mod gmt {
		use chrono::{DateTime, NaiveDateTime, Utc};
		use serde::{self, Deserialize, Deserializer, Serializer};

		#[allow(unused)]
		pub(crate) fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let s = format!("{}", date.format(crate::oss::GMT_DATE_FMT));
			serializer.serialize_str(&s)
		}

		#[allow(unused)]
		pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
		where
			D: Deserializer<'de>,
		{
			let s = String::deserialize(deserializer)?;
			let dt = NaiveDateTime::parse_from_str(&s, crate::oss::GMT_DATE_FMT)
				.unwrap()
				.and_utc();

			Ok(dt)
		}
	}

	pub(crate) mod gmt_option {
		use chrono::{DateTime, NaiveDateTime, Utc};
		use serde::{self, Deserialize, Deserializer, Serializer};

		pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match date {
				Some(date) => {
					let s = format!("{}", date.format(crate::oss::GMT_DATE_FMT));
					serializer.serialize_str(&s)
				}
				None => serializer.serialize_str("null"),
			}
		}

		pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
		where
			D: Deserializer<'de>,
		{
			let s = String::deserialize(deserializer)?;
			let dt = NaiveDateTime::parse_from_str(&s, crate::oss::GMT_DATE_FMT)
				.unwrap()
				.and_utc();

			Ok(Some(dt))
		}
	}
}
