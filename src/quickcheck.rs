//! Implementations of the [`quickcheck::Arbitrary`](quickcheck_dep::Arbitrary) trait.
//!
//! This enables users to write tests such as this, and have test values provided automatically:
//!
//! ```
//! # #![allow(dead_code)]
//! # use quickcheck_dep::quickcheck;
//! # #[cfg(pretend_we_didnt_rename_the_dependency)]
//! use quickcheck::quickcheck;
//! use time::Date;
//!
//! struct DateRange {
//!     from: Date,
//!     to: Date,
//! }
//!
//! impl DateRange {
//!     fn new(from: Date, to: Date) -> Result<Self, ()> {
//!         Ok(DateRange { from, to })
//!     }
//! }
//!
//! quickcheck! {
//!     fn date_range_is_well_defined(from: Date, to: Date) -> bool {
//!         let r = DateRange::new(from, to);
//!         if from <= to {
//!             r.is_ok()
//!         } else {
//!             r.is_err()
//!         }
//!     }
//! }
//! ```
//!
//! An implementation for `Instant` is intentionally omitted since its values are only meaningful in
//! relation to a [`Duration`], and obtaining an `Instant` from a [`Duration`] is very simple
//! anyway.

use crate::{
    date::{MAX_YEAR, MIN_YEAR},
    hack,
    util::days_in_year,
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday,
};
use alloc::boxed::Box;
use quickcheck_dep::{empty_shrinker, single_shrinker, Arbitrary, Gen};

/// Obtain an arbitrary value between the minimum and maximum inclusive.
fn arbitrary_between<T>(g: &mut Gen, min: T, max: T) -> T
where
    T: PartialOrd
        + core::ops::AddAssign
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Rem<Output = T>
        + Arbitrary
        + Copy,
{
    #[allow(clippy::eq_op)]
    let zero = min - min;

    let range = max - min;
    let mut within_range = T::arbitrary(g) % range;

    if within_range < zero {
        within_range += range;
    }

    within_range + min
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Date {
    fn arbitrary(g: &mut Gen) -> Self {
        let year = arbitrary_between(g, MIN_YEAR, MAX_YEAR);
        let ordinal = arbitrary_between(g, 1, days_in_year(year));
        Self::from_ordinal_date_unchecked(year, ordinal)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.to_ordinal_date()
                .shrink()
                .flat_map(|(year, ordinal)| Self::from_ordinal_date(year, ordinal)),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Duration {
    fn arbitrary(g: &mut Gen) -> Self {
        let seconds = i64::arbitrary(g);
        let mut nanoseconds = arbitrary_between(g, 0, 999_999_999);

        // Coerce the sign if necessary. Also allow for the creation of a negative Duration under
        // one second.
        if seconds < 0 || (seconds == 0 && bool::arbitrary(g)) {
            nanoseconds *= -1;
        }

        Self {
            seconds,
            nanoseconds,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.nanoseconds, self.seconds)
                .shrink()
                .map(|(mut nanoseconds, seconds)| {
                    // Coerce the sign if necessary.
                    if (seconds > 0 && nanoseconds < 0) || (seconds < 0 && nanoseconds > 0) {
                        nanoseconds *= -1;
                    }

                    Self {
                        seconds,
                        nanoseconds,
                    }
                }),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Time {
    fn arbitrary(g: &mut Gen) -> Self {
        Self {
            hour: arbitrary_between(g, 0, 23),
            minute: arbitrary_between(g, 0, 59),
            second: arbitrary_between(g, 0, 59),
            nanosecond: arbitrary_between(g, 0, 999_999_999),
            padding: hack::Padding::Optimize,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.as_hms_nano()
                .shrink()
                .map(|(hour, minute, second, nanosecond)| Self {
                    hour,
                    minute,
                    second,
                    nanosecond,
                    padding: hack::Padding::Optimize,
                }),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for PrimitiveDateTime {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(Date::arbitrary(g), Time::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.date, self.time)
                .shrink()
                .map(|(date, time)| Self { date, time }),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for UtcOffset {
    fn arbitrary(g: &mut Gen) -> Self {
        let hours = arbitrary_between(g, -23, 23);
        let mut minutes = arbitrary_between(g, 0, 59);
        let mut seconds = arbitrary_between(g, 0, 59);

        // Coerce the signs if necessary. Also allow for the creation of a negative offset under one
        // hour.
        if hours < 0
            || (hours == 0 && bool::arbitrary(g))
            || (hours == 0 && minutes == 0 && bool::arbitrary(g))
        {
            minutes *= -1;
            seconds *= -1;
        }

        Self {
            hours,
            minutes,
            seconds,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.as_hms()
                .shrink()
                .map(|(hours, mut minutes, mut seconds)| {
                    // Coerce the signs if necessary.
                    if (hours > 0 && minutes < 0) || (hours < 0 && minutes > 0) {
                        minutes *= -1;
                    }
                    if (hours > 0 && seconds < 0)
                        || (hours < 0 && seconds > 0)
                        || (minutes > 0 && seconds < 0)
                        || (minutes < 0 && seconds > 0)
                    {
                        seconds *= -1;
                    }

                    Self {
                        hours,
                        minutes,
                        seconds,
                    }
                }),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for OffsetDateTime {
    fn arbitrary(g: &mut Gen) -> Self {
        let datetime = PrimitiveDateTime::arbitrary(g);
        let offset = UtcOffset::arbitrary(g);
        datetime.assume_offset(offset)
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.utc_datetime.utc_to_offset(self.offset), self.offset)
                .shrink()
                .map(|(utc_datetime, offset)| utc_datetime.assume_offset(offset)),
        )
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "quickcheck")))]
impl Arbitrary for Weekday {
    fn arbitrary(g: &mut Gen) -> Self {
        use Weekday::*;
        match arbitrary_between::<u8>(g, 0, 6) {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            _ => Sunday,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Self::Monday => empty_shrinker(),
            _ => single_shrinker(self.previous()),
        }
    }
}
