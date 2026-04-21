use std::time::Duration;

use clap::Parser;

fn parse_positive_number(value: &str) -> Result<u64, String> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| "must be a whole number".to_string())?;

    if parsed == 0 {
        Err("must be at least 1".to_string())
    } else {
        Ok(parsed)
    }
}

fn parse_duration(value: &str) -> Result<Duration, String> {
    let value = value.trim().to_ascii_lowercase();
    if value.is_empty() {
        return Err("must not be empty".to_string());
    }

    let total_seconds = if let Some((minutes, seconds)) = value.split_once(':') {
        if seconds.contains(':') {
            return Err("use MM:SS format".to_string());
        }

        let minutes = parse_nonnegative_number(minutes, "minutes")?;
        let seconds = parse_nonnegative_number(seconds, "seconds")?;
        if seconds >= 60 {
            return Err("seconds must be less than 60 in MM:SS format".to_string());
        }
        minutes
            .checked_mul(60)
            .and_then(|minutes| minutes.checked_add(seconds))
            .ok_or_else(|| "duration is too large".to_string())?
    } else if value.ends_with('m') || value.ends_with('s') {
        parse_unit_duration(&value)?
    } else {
        parse_positive_number(&value)?
            .checked_mul(60)
            .ok_or_else(|| "duration is too large".to_string())?
    };

    if total_seconds == 0 {
        Err("must be at least 1".to_string())
    } else {
        Ok(Duration::from_secs(total_seconds))
    }
}

fn parse_nonnegative_number(value: &str, label: &str) -> Result<u64, String> {
    if value.is_empty() {
        return Err(format!("{label} must not be empty"));
    }

    value
        .parse::<u64>()
        .map_err(|_| format!("{label} must be a whole number"))
}

fn parse_unit_duration(value: &str) -> Result<u64, String> {
    let mut remaining = value;
    let mut minutes = None;
    let mut seconds = None;

    while !remaining.is_empty() {
        let digit_count = remaining
            .chars()
            .take_while(|char| char.is_ascii_digit())
            .map(char::len_utf8)
            .sum::<usize>();

        if digit_count == 0 || digit_count == remaining.len() {
            return Err("use durations like 15m, 90s, or 1m30s".to_string());
        }

        let (number, rest) = remaining.split_at(digit_count);
        let (unit, rest) = rest.split_at(1);
        let number = parse_nonnegative_number(number, "duration")?;

        match unit {
            "m" if minutes.is_none() && seconds.is_none() => minutes = Some(number),
            "s" if seconds.is_none() => seconds = Some(number),
            _ => return Err("use durations like 15m, 90s, or 1m30s".to_string()),
        }

        remaining = rest;
    }

    minutes
        .unwrap_or(0)
        .checked_mul(60)
        .and_then(|minutes| minutes.checked_add(seconds.unwrap_or(0)))
        .ok_or_else(|| "duration is too large".to_string())
}

#[derive(Debug, Parser)]
#[command(
    name = "timepour",
    version,
    about = "A terminal focus timer with a deterministic tetromino stack"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    #[command(about = "Start a focus session")]
    Start {
        #[arg(
            value_name = "DURATION",
            help = "Focus length: 15, 15m, 90s, 1:30, or 1m30s (default: 25m)",
            value_parser = parse_duration
        )]
        duration: Option<Duration>,
    },
    #[command(about = "Start a break session")]
    Break {
        #[arg(
            value_name = "DURATION",
            help = "Break length: 5, 5m, 90s, 1:30, or 1m30s (default: 5m)",
            value_parser = parse_duration
        )]
        duration: Option<Duration>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::time::Duration;

    #[test]
    fn start_accepts_bare_number_as_minutes() {
        let cli = Cli::try_parse_from(["timepour", "start", "15"]).unwrap();

        match cli.command {
            Commands::Start { duration } => assert_eq!(duration, Some(Duration::from_secs(900))),
            Commands::Break { .. } => panic!("expected start command"),
        }
    }

    #[test]
    fn start_accepts_colon_duration() {
        let cli = Cli::try_parse_from(["timepour", "start", "1:30"]).unwrap();

        match cli.command {
            Commands::Start { duration } => assert_eq!(duration, Some(Duration::from_secs(90))),
            Commands::Break { .. } => panic!("expected start command"),
        }
    }

    #[test]
    fn start_accepts_compact_minute_second_duration() {
        let cli = Cli::try_parse_from(["timepour", "start", "1m30s"]).unwrap();

        match cli.command {
            Commands::Start { duration } => assert_eq!(duration, Some(Duration::from_secs(90))),
            Commands::Break { .. } => panic!("expected start command"),
        }
    }

    #[test]
    fn start_accepts_seconds_duration() {
        let cli = Cli::try_parse_from(["timepour", "start", "90s"]).unwrap();

        match cli.command {
            Commands::Start { duration } => assert_eq!(duration, Some(Duration::from_secs(90))),
            Commands::Break { .. } => panic!("expected start command"),
        }
    }

    #[test]
    fn zero_duration_error_is_clear() {
        let error = Cli::try_parse_from(["timepour", "start", "0"]).unwrap_err();

        assert!(error.to_string().contains("must be at least 1"));
    }

    #[test]
    fn break_accepts_duration() {
        let cli = Cli::try_parse_from(["timepour", "break", "1:30"]).unwrap();

        match cli.command {
            Commands::Break { duration } => assert_eq!(duration, Some(Duration::from_secs(90))),
            Commands::Start { .. } => panic!("expected break command"),
        }
    }

    #[test]
    fn focus_alias_is_not_supported() {
        let result = Cli::try_parse_from(["timepour", "focus", "10"]);

        assert!(result.is_err());
    }

    #[test]
    fn seconds_flag_is_not_supported() {
        let result = Cli::try_parse_from(["timepour", "start", "--seconds", "30"]);

        assert!(result.is_err());
    }
}
