use anyhow::Result;
use chrono::{DateTime, Utc};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::{
    path::{Path, PathBuf},
    time::{Duration, UNIX_EPOCH},
};

pub fn setup_logger(level_filter: LevelFilter) -> Result<()> {
    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            let file_name = record.file().unwrap_or("unknown");
            out.finish(format_args!(
                r###"{{"timestamp":"{}","level":"{}","file":"{}","line":{},"target":"{}","message":"{}"}}"###,
                chrono::Local::now().to_rfc3339(),
                record.level(),
                file_name,
                record.line().unwrap_or(0),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Warn)
        .chain(fern::log_file("logs/output.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            let file_name = record.file().unwrap_or("unknown");
            let colors = ColoredLevelConfig {
                trace: Color::Cyan,
                debug: Color::Magenta,
                info: Color::Green,
                warn: Color::Red,
                error: Color::BrightRed,
                ..ColoredLevelConfig::new()
            };

            out.finish(format_args!(
                "[{} {} {}:{}] {}",
                chrono::Local::now().to_rfc3339(),
                colors.color(record.level()),
                file_name,
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(level_filter)
        .chain(std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
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

/// This function will split the file name and return the path starting from the `src` directory.
/// Only useful for logging.
#[allow(dead_code)]
fn split_file_name(file_name: &str) -> String {
    let path = Path::new(file_name);
    let mut result = PathBuf::new();
    let mut add = false;

    for component in path.components() {
        if add {
            result.push(component);
        } else if component.as_os_str() == "src" {
            add = true;
            result.push(component);
        }
    }

    result.to_str().unwrap_or("").to_string()
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

    #[test]
    fn test_split_file_name() {
        let file_name = "src/utils.rs";
        let expected = "src/utils.rs";
        let actual = split_file_name(file_name);
        assert_eq!(expected, actual);

        let file_name = "/home/user/.cargo/registry/src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/connect.rs";
        let expected = "src/index.crates.io-6f17d22bba15001f/reqwest-0.11.22/src/connect.rs";
        let actual = split_file_name(file_name);
        assert_eq!(expected, actual);

        // Additional tests
        let file_name = "/home/user/src/main.rs";
        let expected = "src/main.rs";
        let actual = split_file_name(file_name);
        assert_eq!(expected, actual);

        let file_name = "/home/src/user/main.rs";
        let expected = "src/user/main.rs";
        let actual = split_file_name(file_name);
        assert_eq!(expected, actual);
    }
}
