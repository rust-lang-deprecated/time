use bench_util::setup_benchmark;
use criterion::BatchSize;
use std::{convert::TryFrom, time::Duration as StdDuration};
use time::{
    ext::{NumericalDuration, NumericalStdDuration},
    Duration,
};

setup_benchmark! {
    "Duration",

    fn unit_values(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::zero(),
            Duration::nanosecond(),
            Duration::microsecond(),
            Duration::millisecond(),
            Duration::second(),
            Duration::minute(),
            Duration::hour(),
            Duration::day(),
            Duration::week(),
        ));
    }

    fn is_zero(ben: &mut Bencher) {
        let a = (-1).nanoseconds();
        let b = 0.seconds();
        let c = 1.nanoseconds();
        ben.iter(|| (
            a.is_zero(),
            b.is_zero(),
            c.is_zero(),
        ));
    }

    fn is_negative(ben: &mut Bencher) {
        let a = (-1).seconds();
        let b = 0.seconds();
        let c = 1.seconds();
        ben.iter(|| (
            a.is_negative(),
            b.is_negative(),
            c.is_negative(),
        ));
    }

    fn is_positive(ben: &mut Bencher) {
        let a = (-1).seconds();
        let b = 0.seconds();
        let c = 1.seconds();
        ben.iter(|| (
            a.is_positive(),
            b.is_positive(),
            c.is_positive(),
        ));
    }

    fn abs(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 0.seconds();
        let c = (-1).seconds();
        ben.iter(|| (
            a.abs(),
            b.abs(),
            c.abs(),
        ));
    }

    fn new(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::new(1, 0),
            Duration::new(-1, 0),
            Duration::new(1, 2_000_000_000),

            Duration::new(0, 0),
            Duration::new(0, 1_000_000_000),
            Duration::new(-1, 1_000_000_000),
            Duration::new(-2, 1_000_000_000),

            Duration::new(1, -1),
            Duration::new(-1, 1),
            Duration::new(1, 1),
            Duration::new(-1, -1),
            Duration::new(0, 1),
            Duration::new(0, -1),

            Duration::new(-1, 1_400_000_000),
            Duration::new(-2, 1_400_000_000),
            Duration::new(-3, 1_400_000_000),
            Duration::new(1, -1_400_000_000),
            Duration::new(2, -1_400_000_000),
            Duration::new(3, -1_400_000_000),
        ));
    }

    fn weeks(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::weeks(1),
            Duration::weeks(2),
            Duration::weeks(-1),
            Duration::weeks(-2),
        ));
    }

    fn whole_weeks(ben: &mut Bencher) {
        let a = Duration::weeks(1);
        let b = Duration::weeks(-1);
        let c = Duration::days(6);
        let d = Duration::days(-6);
        ben.iter(|| (
            a.whole_weeks(),
            b.whole_weeks(),
            c.whole_weeks(),
            d.whole_weeks(),
        ));
    }

    fn days(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::days(1),
            Duration::days(2),
            Duration::days(-1),
            Duration::days(-2),
        ));
    }

    fn whole_days(ben: &mut Bencher) {
        let a = Duration::days(1);
        let b = Duration::days(-1);
        let c = Duration::hours(23);
        let d = Duration::hours(-23);
        ben.iter(|| (
            a.whole_days(),
            b.whole_days(),
            c.whole_days(),
            d.whole_days(),
        ));
    }

    fn hours(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::hours(1),
            Duration::hours(2),
            Duration::hours(-1),
            Duration::hours(-2),
        ));
    }

    fn whole_hours(ben: &mut Bencher) {
        let a = Duration::hours(1);
        let b = Duration::hours(-1);
        let c = Duration::minutes(59);
        let d = Duration::minutes(-59);
        ben.iter(|| (
            a.whole_hours(),
            b.whole_hours(),
            c.whole_hours(),
            d.whole_hours(),
        ));
    }

    fn minutes(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::minutes(1),
            Duration::minutes(2),
            Duration::minutes(-1),
            Duration::minutes(-2),
        ));
    }

    fn whole_minutes(ben: &mut Bencher) {
        let a = 1.minutes();
        let b = (-1).minutes();
        let c = 59.seconds();
        let d = (-59).seconds();
        ben.iter(|| (
            a.whole_minutes(),
            b.whole_minutes(),
            c.whole_minutes(),
            d.whole_minutes(),
        ));
    }

    fn seconds(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::seconds(1),
            Duration::seconds(2),
            Duration::seconds(-1),
            Duration::seconds(-2),
        ));
    }

    fn whole_seconds(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.minutes();
        let d = (-1).minutes();
        ben.iter(|| (
            a.whole_seconds(),
            b.whole_seconds(),
            c.whole_seconds(),
            d.whole_seconds(),
        ));
    }

    fn seconds_f64(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::seconds_f64(0.5),
            Duration::seconds_f64(-0.5),
        ));
    }

    fn as_seconds_f64(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.minutes();
        let d = (-1).minutes();
        let e = 1.5.seconds();
        let f = (-1.5).seconds();
        ben.iter(|| (
            a.as_seconds_f64(),
            b.as_seconds_f64(),
            c.as_seconds_f64(),
            d.as_seconds_f64(),
            e.as_seconds_f64(),
            f.as_seconds_f64(),
        ));
    }

    fn seconds_f32(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::seconds_f32(0.5),
            Duration::seconds_f32(-0.5),
        ));
    }

    fn as_seconds_f32(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.minutes();
        let d = (-1).minutes();
        let e = 1.5.seconds();
        let f = (-1.5).seconds();
        ben.iter(|| (
            a.as_seconds_f32(),
            b.as_seconds_f32(),
            c.as_seconds_f32(),
            d.as_seconds_f32(),
            e.as_seconds_f32(),
            f.as_seconds_f32(),
        ));
    }

    fn milliseconds(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::milliseconds(1),
            Duration::milliseconds(-1),
        ));
    }

    fn whole_milliseconds(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 1.milliseconds();
        let d = (-1).milliseconds();
        ben.iter(|| (
            a.whole_milliseconds(),
            b.whole_milliseconds(),
            c.whole_milliseconds(),
            d.whole_milliseconds(),
        ));
    }

    fn subsec_milliseconds(ben: &mut Bencher) {
        let a = 1.4.seconds();
        let b = (-1.4).seconds();
        ben.iter(|| (
            a.subsec_milliseconds(),
            b.subsec_milliseconds(),
        ));
    }

    fn microseconds(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::microseconds(1),
            Duration::microseconds(-1),
        ));
    }

    fn whole_microseconds(ben: &mut Bencher) {
        let a = 1.milliseconds();
        let b = (-1).milliseconds();
        let c = 1.microseconds();
        let d = (-1).microseconds();
        ben.iter(|| (
            a.whole_microseconds(),
            b.whole_microseconds(),
            c.whole_microseconds(),
            d.whole_microseconds(),
        ));
    }

    fn subsec_microseconds(ben: &mut Bencher) {
        let a = 1.0004.seconds();
        let b = (-1.0004).seconds();
        ben.iter(|| (
            a.subsec_microseconds(),
            b.subsec_microseconds(),
        ));
    }

    fn nanoseconds(ben: &mut Bencher) {
        ben.iter(|| (
            Duration::nanoseconds(1),
            Duration::nanoseconds(-1),
        ));
    }

    fn whole_nanoseconds(ben: &mut Bencher) {
        let a = 1.microseconds();
        let b = (-1).microseconds();
        let c = 1.nanoseconds();
        let d = (-1).nanoseconds();
        ben.iter(|| (
            a.whole_nanoseconds(),
            b.whole_nanoseconds(),
            c.whole_nanoseconds(),
            d.whole_nanoseconds(),
        ));
    }

    fn subsec_nanoseconds(ben: &mut Bencher) {
        let a = 1.000_000_4.seconds();
        let b = (-1.000_000_4).seconds();
        ben.iter(|| (
            a.subsec_nanoseconds(),
            b.subsec_nanoseconds(),
        ));
    }

    fn checked_add(ben: &mut Bencher) {
        let a = 5.seconds();
        let b = Duration::max_value();
        let c = (-5).seconds();

        let a2 = 5.seconds();
        let b2 = 1.nanoseconds();
        let c2 = 5.seconds();

        ben.iter(|| (
            a.checked_add(a2),
            b.checked_add(b2),
            c.checked_add(c2),
        ));
    }

    fn checked_sub(ben: &mut Bencher) {
        let a = 5.seconds();
        let b = Duration::min_value();
        let c = 5.seconds();

        let a2 = 5.seconds();
        let b2 = 1.nanoseconds();
        let c2 = 10.seconds();

        ben.iter(|| (
            a.checked_sub(a2),
            b.checked_sub(b2),
            c.checked_sub(c2),
        ));
    }

    fn checked_mul(ben: &mut Bencher) {
        let d = 5.seconds();
        ben.iter(|| d.checked_mul(2));
    }

    fn checked_div(ben: &mut Bencher) {
        let d = 10.seconds();
        ben.iter(|| d.checked_div(2));
    }

    fn try_from_std_duration(ben: &mut Bencher) {
        let a = 0.std_seconds();
        let b = 1.std_seconds();
        ben.iter(|| (
            Duration::try_from(a),
            Duration::try_from(b),
        ));
    }

    fn try_to_std_duration(ben: &mut Bencher) {
        let a = 0.seconds();
        let b = 1.seconds();
        let c = (-1).seconds();
        ben.iter(|| (
            StdDuration::try_from(a),
            StdDuration::try_from(b),
            StdDuration::try_from(c),
        ));
    }

    fn add(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 2.seconds();
        let c = 500.milliseconds();
        let d = (-1).seconds();
        ben.iter(|| a + b + c + d);
    }

    fn add_std(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 2.std_seconds();
        ben.iter(|| a + b);
    }

    fn std_add(ben: &mut Bencher) {
        let a = 1.std_seconds();
        let b = 2.seconds();
        ben.iter(|| a + b);
    }

    fn add_assign(ben: &mut Bencher) {
        let dta = 1.seconds();
        let dtb = 500.milliseconds();
        let dtc = (-1).seconds();
        ben.iter_batched_ref(
            || 1.seconds(),
            |duration| {
                *duration += dta;
                *duration += dtb;
                *duration += dtc;
            },
            BatchSize::SmallInput
        );
    }

    fn add_assign_std(ben: &mut Bencher) {
        let dta = 1.std_seconds();
        let dtb = 500.std_milliseconds();
        ben.iter_batched_ref(
            || 1.seconds(),
            |duration| {
                *duration += dta;
                *duration += dtb;
            },
            BatchSize::SmallInput
        );
    }

    fn neg(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = (-1).seconds();
        let c = 0.seconds();
        ben.iter(|| (-a, -b, -c));
    }

    fn sub(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 1.seconds();
        let c = 1_500.milliseconds();
        let d = 500.milliseconds();
        let e = 1.seconds();
        let f = (-1).seconds();
        ben.iter(|| a - b - c - d - e - f);
    }

    fn sub_std(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 2.std_seconds();
        ben.iter(|| a - b);
    }

    fn std_sub(ben: &mut Bencher) {
        let a = 1.std_seconds();
        let b = 2.seconds();
        ben.iter(|| a - b);
    }

    fn sub_assign(ben: &mut Bencher) {
        let dta = 1.seconds();
        let dtb = 500.milliseconds();
        let dtc = (-1).seconds();
        ben.iter_batched_ref(
            || 1.seconds(),
            |duration| {
                *duration -= dta;
                *duration -= dtb;
                *duration -= dtc;
            },
            BatchSize::SmallInput
        );
    }

    fn mul_int(ben: &mut Bencher) {
        let d = 1.seconds();
        ben.iter(|| (d * 2) * -2);
    }

    fn mul_int_assign(ben: &mut Bencher) {
        ben.iter_batched_ref(
            || 1.seconds(),
            |duration| {
                *duration *= 2;
                *duration *= -2;
            },
            BatchSize::SmallInput
        );
    }

    fn int_mul(ben: &mut Bencher) {
        let d = 1.seconds();
        ben.iter(|| (-2) * (2 * d));
    }

    fn div_int(ben: &mut Bencher) {
        let d = 1.seconds();
        ben.iter(|| (d / 2) / -2);
    }

    fn div_int_assign(ben: &mut Bencher) {
        ben.iter_batched_ref(
            || 1.seconds(),
            |duration| {
                *duration /= 2;
                *duration /= -2;
            },
            BatchSize::SmallInput
        );
    }

    fn div(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 0.5.seconds();
        ben.iter(|| a / b);
    }

    fn mul_float(ben: &mut Bencher) {
        let d = 1.seconds();
        ben.iter(||
            d * 1.5_f32 * 2.5_f32 * -1.5_f32 * 0_f32 * 1.5_f64 * 2.5_f64 * -1.5_f64 * 0_f64
        );
    }

    fn float_mul(ben: &mut Bencher) {
        let d = 1.seconds();
        ben.iter(||
            1.5_f32 * (2.5_f32 * (-1.5_f32 * (3.15_f32 * (1.5_f64 * (2.5_f64 * (-1.5_f64 * d))))))
        );
    }

    fn mul_float_assign(ben: &mut Bencher) {
        ben.iter_batched_ref(
            || 1.seconds(),
            |duration| {
                *duration *= 1.5_f32;
                *duration *= 2.5_f32;
                *duration *= -1.5_f32;
                *duration *= 3.15_f32;
                *duration *= 1.5_f64;
                *duration *= 2.5_f64;
                *duration *= -1.5_f64;
                *duration *= 0_f64;
            },
            BatchSize::SmallInput
        );
    }

    fn div_float(ben: &mut Bencher) {
        let d = 1.seconds();
        ben.iter(|| d / 1_f32 / 2_f32 / -1_f32 / 1_f64 / 2_f64 / -1_f64);
    }

    fn div_float_assign(ben: &mut Bencher) {
        ben.iter_batched_ref(
            || 10.seconds(),
            |duration| {
                *duration /= 1_f32;
                *duration /= 2_f32;
                *duration /= -1_f32;
                *duration /= 1_f64;
                *duration /= 2_f64;
                *duration /= -1_f64;
            },
            BatchSize::SmallInput
        );
    }

    fn partial_eq(ben: &mut Bencher) {
        let a = 1.minutes();
        let b = (-1).minutes();
        let c = 40.seconds();
        ben.iter(|| (a == b, c == a));
    }

    fn partial_eq_std(ben: &mut Bencher) {
        let a = (-1).seconds();
        let b = 1.std_seconds();
        let c = (-1).minutes();
        let d = 1.std_minutes();
        let e = 40.seconds();
        ben.iter(|| (
            a == b,
            c == d,
            e == d,
        ));
    }

    fn std_partial_eq(ben: &mut Bencher) {
        let a = 1.std_seconds();
        let b = (-1).seconds();
        let c = 1.std_minutes();
        let d = (-1).minutes();
        let e = 40.std_seconds();
        let f = 1.minutes();
        ben.iter(|| (
            a == b,
            c == d,
            e == f,
        ));
    }

    fn partial_ord(ben: &mut Bencher) {
        let a = 0.seconds();
        let b = 1.seconds();
        let c = (-1).seconds();
        let d = 1.minutes();
        let e = (-1).minutes();
        ben.iter(|| (
            a.partial_cmp(&a),
            b.partial_cmp(&a),
            b.partial_cmp(&c),
            c.partial_cmp(&b),
            a.partial_cmp(&c),
            a.partial_cmp(&b),
            c.partial_cmp(&a),
            d.partial_cmp(&b),
            e.partial_cmp(&c),
        ));
    }

    fn partial_ord_std(ben: &mut Bencher) {
        let a = 0.seconds();
        let b = 0.std_seconds();
        let c = 1.seconds();
        let d = (-1).seconds();
        let e = 1.std_seconds();
        let f = 1.minutes();
        let g = u64::MAX.std_seconds();
        ben.iter(|| (
            a.partial_cmp(&b),
            c.partial_cmp(&b),
            d.partial_cmp(&e),
            a.partial_cmp(&e),
            d.partial_cmp(&b),
            f.partial_cmp(&e),
            a.partial_cmp(&g),
        ));
    }

    fn std_partial_ord(ben: &mut Bencher) {
        let a = 0.std_seconds();
        let b = 0.seconds();
        let c = 1.std_seconds();
        let d = (-1).seconds();
        let e = 1.seconds();
        let f = 1.std_minutes();
        ben.iter(|| (
            a.partial_cmp(&b),
            c.partial_cmp(&b),
            c.partial_cmp(&d),
            a.partial_cmp(&d),
            a.partial_cmp(&e),
            f.partial_cmp(&e),
        ));
    }

    fn ord(ben: &mut Bencher) {
        let a = 1.seconds();
        let b = 0.seconds();
        let c = (-1).seconds();
        let d = 1.minutes();
        let e = (-1).minutes();
        ben.iter(|| (
            a > b,
            a > c,
            c < a,
            b > c,
            b < a,
            c < b,
            d > a,
            e < c,
        ));
    }
}
