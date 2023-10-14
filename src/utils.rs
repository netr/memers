use anyhow::Result;
use chrono::{DateTime, Utc};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::time::{Duration, UNIX_EPOCH};

pub fn setup_logger() -> Result<()> {
    let colors = ColoredLevelConfig {
        trace: Color::Cyan,
        debug: Color::Magenta,
        info: Color::Green,
        warn: Color::Red,
        error: Color::BrightRed,
        ..ColoredLevelConfig::new()
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout())
        .level(log::LevelFilter::Error)
        .level_for("memers", LevelFilter::Info)
        .apply()?;

    Ok(())
}

pub fn convert_utc_to_str(timestamp: u64) -> String {
    let timestamp = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Utc>::from(timestamp);
    datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

pub fn convert_utc_to_local_str(timestamp: u64) -> String {
    let timestamp = UNIX_EPOCH + Duration::from_secs(timestamp);
    let datetime = DateTime::<Utc>::from(timestamp).with_timezone(&chrono::Local);
    datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_should_convert_utc_to_local_str() {
        // Ignored because this test will fail if you're not in the same time zone as me.
        // Your time zone: Wednesday, August 25, 2021 7:00:00 AM GMT-07:00 DST
        let timestamp = 1629900000;
        let expected = "2021-08-25 07:00:00.000000000";
        let actual = convert_utc_to_local_str(timestamp);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_should_convert_utc_to_str() {
        // GMT: Wednesday, August 25, 2021 2:00:00 PM
        let timestamp = 1629900000;
        let expected = "2021-08-25 14:00:00.000000000";
        let actual = convert_utc_to_str(timestamp);
        assert_eq!(expected, actual);
    }
}
