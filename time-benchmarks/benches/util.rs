use bench_util::setup_benchmark;
use time::util;

setup_benchmark! {
    "Utils",

    fn is_leap_year(ben: &mut Bencher) {
        ben.iter(|| (
            util::is_leap_year(1900),
            util::is_leap_year(2000),
            util::is_leap_year(2004),
            util::is_leap_year(2005),
            util::is_leap_year(2100),
        ));
    }

    fn days_in_year(ben: &mut Bencher) {
        ben.iter(|| (
            util::days_in_year(1900),
            util::days_in_year(2000),
            util::days_in_year(2004),
            util::days_in_year(2005),
            util::days_in_year(2100),
        ));
    }

    fn weeks_in_year(ben: &mut Bencher) {
        ben.iter(|| (
            util::weeks_in_year(2019),
            util::weeks_in_year(2020),
        ));
    }

    fn validate_format_string(ben: &mut Bencher) {
        ben.iter(|| (
            util::validate_format_string("%H foo"),
            util::validate_format_string("%H%%"),
            util::validate_format_string("%"),
        ));
    }
}
