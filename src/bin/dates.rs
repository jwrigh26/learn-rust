use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use colored::*;

use chrono_tz::America::Denver;
use chrono_tz::Tz;

const DATE_STRING: &str = "2023-10-09T10:20:30Z";

fn main() {
    let denver_date_string = "2023-10-11T22:33:16";
    // Parse the string into a DateTime object in the Denver timezone
    // let denver_datetime = Denver
    //     .datetime_from_str(&denver_date_string, "%Y-%m-%dT%H:%M:%S")
    //     .expect("Invalid date string");

    // let denver_datetime: DateTime<FixedOffset> =
    //     DateTime::parse_from_str(&denver_date_string, "%Y-%m-%dT%H:%M:%S%:z")
    //         .expect("Invalid date string");

    // let denver_date_string = "2023-10-09T12:34:56";  // example date string
    let naive_datetime = NaiveDateTime::parse_from_str(&denver_date_string, "%Y-%m-%dT%H:%M:%S")
        .expect("Invalid date string");
    
    let denver_datetime = Denver.from_local_datetime(&naive_datetime).single().expect("Invalid date string");

    // Convert the Denver DateTime object to a UTC DateTime object
    let utc_datetime: DateTime<chrono::Utc> = denver_datetime.with_timezone(&chrono::Utc);

    // Format the UTC DateTime object as an ISO 8601 string
    let iso8601_string = utc_datetime.to_rfc3339();

    println!("");
    println!("----------------------------------------");
    println!("");

    println!("Denver DateTime: {}", denver_datetime);
    println!("UTC DateTime: {}", utc_datetime);
    println!("ISO 8601 String: {}", iso8601_string);

    println!("");
    println!("----------------------------------------");
    println!("");
    println!(
        "{}",
        format!("{}", "Important stuff Starts Here").yellow().bold()
    );
    println!("");
    println!("----------------------------------------");
    println!("");
    println!(
        "{}",
        format!("{}", "Dates and Times and Other Things")
            .blue()
            .bold()
    );
    println!("");
    let now: DateTime<Utc> = Utc::now();
    println!("The value of now is: {}", format!("{}", now).green());

    let date_time: DateTime<Utc> = "2023-10-09T10:20:30Z".parse().unwrap();
    println!(
        "The dateString:{} is: {}",
        format!("{}", DATE_STRING).yellow(),
        format!("{}", date_time).blue()
    );

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Now: formatted => {}", format!("{}", formatted).green());

    let parsed = DateTime::parse_from_rfc3339("2023-10-09T10:20:30Z");
    println!("2023-10-09T10:20:30Z: parsed => {}", parsed.unwrap());

    // NaiveDateTime is a type provided by the chrono crate in Rust.
    // The term "naive" here is used to indicate that this type of date-time object
    // doesn't have any timezone information associated with it. It's just a plain
    // date and time, without any awareness of where in the world it's supposed to be.
    let dt: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    println!("");
    println!("----------------------------------------");
    println!("");
    println!("NaiveDate: {}", format!("{}", dt).blue());

    // In the provided date string "2023-10-12T04:18:27+0000", the "+0000"
    // at the end is the timezone offset from UTC, indicating that the date
    // and time are already in UTC. When parsing this string into a DateTime<Utc>
    // object in Rust using "chrono" crate, you can use the `DateTime::parse_from_str`
    // method directly without having to go through a `NaiveDateTime` object first.
    // Hers how you can do it:
    let date_string = "2023-10-12T04:18:27+0000";
    let utc_datetime = DateTime::parse_from_str(&date_string, "%+").expect("Invalid date string");
    println!("");
    println!("----------------------------------------");
    println!("");
    println!("utc_datetime: {}", format!("{}", utc_datetime).blue());

    // In this code:
    // 1. The `use chrono::{DateTime, Utc};` statement imports the `DateTime` and `Utc` types
    // 2. The `date_string` variable is set to the date string: "2023-10-12T04:18:27+0000"
    // 3. The `DateTime::parse_from_str` method is called with the date string and the format
    //    string `"%+"`. The `"%+"` format string tells `chrono` to parse the date string
    //    as an RFC 3339 and ISO 8601 date string and time string, which includes a timezone offset.
    // 4. The `expect` method is called to handle any errors that might occur during parsing.
    //    If parsing succeeds, `utc_datetime` will be a `DateTime<Utc>` object representing
    //    the parsed date and time.
    // 5. The `println!` macro is used to print the `utc_datetime` object to the console.

    // Assuming this is your UTC date string with No Timezone Awareness
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

    println!("");
    println!("Using a local time with no timezone awareness");
    println!("UTC Date and Time: {}", format!("{}", utc_datetime).red());
    println!(
        "New York Date and Time: {}",
        format!("{}", ny_datetime).purple()
    );
    println!("Utah Date and Time: {}", format!("{}", ut_datetime).blue());
    println!("");
    println!("----------------------------------------");
    println!("");
    println!(
        "{}",
        format!("{}", "Important stuff Starts Here").yellow().bold()
    );
    println!("");
}
