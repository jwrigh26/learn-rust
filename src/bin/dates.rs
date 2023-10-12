use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use colored::*;

use chrono_tz::Tz;

fn main() {
    let now: DateTime<Utc> = Utc::now();
    println!("The value of now is: {}", format!("{}", now).green());

    let date_time: DateTime<Utc> = "2023-10-09T10:20:30Z".parse().unwrap();
    println!("{}", date_time);

    // NaiveDateTime is a type provided by the chrono crate in Rust.
    // The term "naive" here is used to indicate that this type of date-time object
    // doesn't have any timezone information associated with it. It's just a plain
    // date and time, without any awareness of where in the world it's supposed to be.
    let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    println!("NaiveDate {}", dt);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("{}", formatted);

    let parsed = DateTime::parse_from_rfc3339("2023-10-09T10:20:30Z");
    println!("{}", parsed.unwrap());

    // Assuming this is your UTC date string
    // let utc_date_string = "2023-10-09T10:20:30";
    let utc_date_string = "2023-10-10T15:12:13";

    // deprecated -- Do not use
    // Parse the string into a DateTime object
    // let utc_datetime = Utc
    //     .datetime_from_str(&utc_date_string, "%Y-%m-%dT%H:%M:%S")
    //     .expect("Invalid date string");

    

    // Breakdown:
    // 1. No Timezone Awareness:
    //    - A NaiveDateTime object simply represents a date and time, without any
    //      reference to a particular time zone. It's "naive" because it doesn't
    //      know about time zones or daylight saving time changes.
    //
    // 2. Usage:
    //    - This type is useful when you don't need to worry about time zones, or
    //      when you're working with date and time data in a context where the time
    //      zone is always the same and doesn't need to be specified.
    //
    // 3. Conversion:
    //    - If you need to work with time zones, you can convert a NaiveDateTime to
    //      a DateTime object, which does have timezone awareness. This conversion
    //      requires you to specify which time zone the NaiveDateTime should be
    //      interpreted in.
    //
    // 4. Example:
    //    - For instance, "2023-10-09 10:20:30" is a naive date and time because it
    //      doesn't specify whether it's 10:20:30 in London, New York, Tokyo, or
    //      anywhere else.
    //
    // 5. Comparison:
    //    - On the other hand, a DateTime object would represent a date and time in
    //      a specific time zone, like "2023-10-09 10:20:30 UTC" or
    //      "2023-10-09 10:20:30 EST".
    //
    // 6. Parsing:
    //    - When parsing a string into a NaiveDateTime, you don't include any
    //      timezone information in the string or the format specifier. If the string
    //      does include timezone information, you would parse it into a DateTime
    //      object instead.

    // Understanding the difference between naive and timezone-aware date and time
    // types is crucial when working with dates and times in Rust, especially when
    // dealing with data from different time zones or when daylight saving time
    // changes might be involved.

    let naive_datetime =
        NaiveDateTime::parse_from_str(&utc_date_string, "%Y-%m-%dT%H:%M:%S").unwrap();
    let utc_datetime = Utc.from_local_datetime(&naive_datetime).unwrap();

    // Convert the DateTime object to a timezone-aware DateTime object using chrono-tz
    // Let's say you want to convert it to New York time
    let ny_timezone: Tz = "America/New_York".parse().expect("Invalid timezone");
    let ny_datetime = utc_datetime.with_timezone(&ny_timezone);

    let ut_timezone: Tz = "America/Denver".parse().expect("Invalid timezone");
    let ut_datetime = utc_datetime.with_timezone(&ut_timezone);

    println!("UTC Date and Time: {}", utc_datetime);
    println!("New York Date and Time: {}", ny_datetime);
    println!("Utah Date and Time: {}", ut_datetime);
}
