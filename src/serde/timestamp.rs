//! Treat an [`OffsetDateTime`] as a [Unix timestamp] for the purposes of serde.
//!
//! Use this module in combination with serde's [`#[with]`][with] attribute.
//!
//! When deserializing, the offset is assumed to be UTC.
//!
//! ```rust
//! # use time::OffsetDateTime;
//! # use time_macros::datetime;
//! # use serde_json::json;
//! # use serde::{Serialize, Deserialize};
//! #[derive(Debug, PartialEq, Serialize, Deserialize)]
//! struct S {
//!     #[serde(with = "time::serde::timestamp")]
//!     datetime: OffsetDateTime,
//! }
//!
//! let s = S {
//!     datetime: datetime!("2019-01-01 0:00 UTC"),
//! };
//! let v = json!({ "datetime": 1_546_300_800 });
//! assert_eq!(v, serde_json::to_value(&s)?);
//! assert_eq!(s, serde_json::from_value(v)?);
//! # Ok::<_, serde_json::Error>(())
//! ```
//!
//! [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
//! [with]: https://serde.rs/field-attrs.html#with

use crate::OffsetDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
struct Wrapper(i64);

pub fn serialize<S: Serializer>(
    datetime: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    Wrapper(datetime.unix_timestamp()).serialize(serializer)
}

pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<OffsetDateTime, D::Error> {
    Wrapper::deserialize(deserializer)
        .map(|Wrapper(timestamp)| timestamp)
        .and_then(|timestamp| {
            OffsetDateTime::from_unix_timestamp(timestamp)
                .map_err(<D::Error as serde::de::Error>::custom)
        })
}

/// Treat an `Option<OffsetDateTime>` as a [Unix timestamp] for the purposes of
/// serde.
///
/// Use this module in combination with serde's [`#[with]`][with] attribute.
///
/// When deserializing, the offset is assumed to be UTC.
///
/// ```rust
/// # use time::OffsetDateTime;
/// # use time_macros::datetime;
/// # use serde_json::json;
/// # use serde::{Serialize, Deserialize};
/// #[derive(Debug, PartialEq, Serialize, Deserialize)]
/// struct S {
///     #[serde(with = "time::serde::timestamp::option")]
///     datetime: Option<OffsetDateTime>,
/// }
///
/// let s = S {
///     datetime: Some(datetime!("2019-01-01 0:00 UTC")),
/// };
/// let v = json!({ "datetime": 1_546_300_800 });
/// assert_eq!(v, serde_json::to_value(&s)?);
/// assert_eq!(s, serde_json::from_value(v)?);
///
/// let s = S { datetime: None };
/// let v = json!({ "datetime": null });
/// assert_eq!(v, serde_json::to_value(&s)?);
/// assert_eq!(s, serde_json::from_value(v)?);
/// # Ok::<_, serde_json::Error>(())
/// ```
///
/// [Unix timestamp]: https://en.wikipedia.org/wiki/Unix_time
/// [with]: https://serde.rs/field-attrs.html#with
pub mod option {
    #[allow(clippy::wildcard_imports)]
    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    struct Wrapper(#[serde(with = "super")] OffsetDateTime);

    pub fn serialize<S: Serializer>(
        option: &Option<OffsetDateTime>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        option.map(Wrapper).serialize(serializer)
    }

    pub fn deserialize<'a, D: Deserializer<'a>>(
        deserializer: D,
    ) -> Result<Option<OffsetDateTime>, D::Error> {
        Option::deserialize(deserializer).map(|opt| opt.map(|Wrapper(datetime)| datetime))
    }
}
