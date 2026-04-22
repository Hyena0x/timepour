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
    } else if value.ends_with('h') || value.ends_with('m') || value.ends_with('s') {
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
    let mut hours = None;
    let mut minutes = None;
    let mut seconds = None;

    while !remaining.is_empty() {
        let digit_count = remaining
            .chars()
            .take_while(|char| char.is_ascii_digit())
            .map(char::len_utf8)
            .sum::<usize>();

        if digit_count == 0 || digit_count == remaining.len() {
            return Err("use durations like 15m, 90s, 1h30m, or 1m30s".to_string());
        }

        let (number, rest) = remaining.split_at(digit_count);
        let (unit, rest) = rest.split_at(1);
        let number = parse_nonnegative_number(number, "duration")?;

        match unit {
            "h" if hours.is_none() && minutes.is_none() && seconds.is_none() => {
                hours = Some(number)
            }
            "m" if minutes.is_none() && seconds.is_none() => minutes = Some(number),
            "s" if seconds.is_none() => seconds = Some(number),
            _ => return Err("use durations like 15m, 90s, 1h30m, or 1m30s".to_string()),
        }

        remaining = rest;
    }

    hours
        .unwrap_or(0)
        .checked_mul(60 * 60)
        .and_then(|hours| {
            minutes
                .unwrap_or(0)
                .checked_mul(60)
                .and_then(|minutes| hours.checked_add(minutes))
        })
        .and_then(|total| total.checked_add(seconds.unwrap_or(0)))
        .ok_or_else(|| "duration is too large".to_string())
}

#[derive(Debug, Parser)]
#[command(
    name = "timepour",
    version,
    about = "A terminal focus timer with a deterministic tetromino stack"
)]
pub struct Cli {
    #[arg(
        value_name = "DURATION",
        help = "Session length: 25, 25m, 90s, 1:30, 1h30m, or 1m30s (default: 25m focus, 5m break)",
        value_parser = parse_duration
    )]
    pub duration: Option<Duration>,

    #[arg(short = 'b', long = "break", help = "Start a break session")]
    pub break_mode: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use std::time::Duration;

    #[test]
    fn top_level_accepts_bare_number_as_focus_minutes() {
        let cli = Cli::try_parse_from(["timepour", "25"]).unwrap();

        assert!(!cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(25 * 60)));
    }

    #[test]
    fn top_level_accepts_explicit_minutes() {
        let cli = Cli::try_parse_from(["timepour", "25m"]).unwrap();

        assert!(!cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(25 * 60)));
    }

    #[test]
    fn top_level_accepts_hours_and_minutes() {
        let cli = Cli::try_parse_from(["timepour", "1h30m"]).unwrap();

        assert!(!cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(90 * 60)));
    }

    #[test]
    fn top_level_still_accepts_colon_duration() {
        let cli = Cli::try_parse_from(["timepour", "1:30"]).unwrap();

        assert!(!cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(90)));
    }

    #[test]
    fn top_level_accepts_seconds_duration() {
        let cli = Cli::try_parse_from(["timepour", "90s"]).unwrap();

        assert!(!cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(90)));
    }

    #[test]
    fn omitted_duration_stays_focus_with_default_duration() {
        let cli = Cli::try_parse_from(["timepour"]).unwrap();

        assert!(!cli.break_mode);
        assert_eq!(cli.duration, None);
    }

    #[test]
    fn zero_duration_error_is_clear() {
        let error = Cli::try_parse_from(["timepour", "0"]).unwrap_err();

        assert!(error.to_string().contains("must be at least 1"));
    }

    #[test]
    fn long_break_flag_selects_break_mode() {
        let cli = Cli::try_parse_from(["timepour", "--break", "5"]).unwrap();

        assert!(cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(5 * 60)));
    }

    #[test]
    fn short_break_flag_selects_break_mode() {
        let cli = Cli::try_parse_from(["timepour", "-b", "5m"]).unwrap();

        assert!(cli.break_mode);
        assert_eq!(cli.duration, Some(Duration::from_secs(5 * 60)));
    }

    #[test]
    fn break_flag_can_use_default_duration() {
        let cli = Cli::try_parse_from(["timepour", "--break"]).unwrap();

        assert!(cli.break_mode);
        assert_eq!(cli.duration, None);
    }

    #[test]
    fn start_subcommand_is_not_supported() {
        let result = Cli::try_parse_from(["timepour", "start", "10"]);

        assert!(result.is_err());
    }

    #[test]
    fn break_subcommand_is_not_supported() {
        let result = Cli::try_parse_from(["timepour", "break", "10"]);

        assert!(result.is_err());
    }

    #[test]
    fn seconds_flag_is_not_supported() {
        let result = Cli::try_parse_from(["timepour", "--seconds", "30"]);

        assert!(result.is_err());
    }
}
