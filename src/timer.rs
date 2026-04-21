use std::time::{Duration, Instant};

/// Represents the type of Pomodoro session currently active.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionKind {
    /// A high-focus work block, typically 25 minutes.
    Focus,
    /// A shorter restorative break block, typically 5 minutes.
    Break,
}

impl SessionKind {
    /// Returns the default duration in minutes for the given session kind.
    pub fn default_minutes(self) -> u64 {
        match self {
            SessionKind::Focus => 25,
            SessionKind::Break => 5,
        }
    }
}

/// The core state tracking mechanism for a countdown timer.
///
/// Manages pause/resume logic and the calculation of remaining time
/// and progress percentage regardless of the current system clock jitter.
#[derive(Debug, Clone)]
pub struct ActiveTimer {
    kind: SessionKind,
    total_duration: Duration,
    started_at: Instant,
    paused_total: Duration,
    paused_at: Option<Instant>,
}

impl ActiveTimer {
    /// Creates a new `ActiveTimer` instance.
    ///
    /// # Arguments
    /// * `kind` - The type of session (Focus/Break).
    /// * `duration` - Optional explicit duration override.
    /// * `started_at` - The absolute start timestamp.
    pub fn new(kind: SessionKind, duration: Option<Duration>, started_at: Instant) -> Self {
        let total_duration =
            duration.unwrap_or_else(|| Duration::from_secs(kind.default_minutes() * 60));

        Self {
            kind,
            total_duration,
            started_at,
            paused_total: Duration::ZERO,
            paused_at: None,
        }
    }

    /// Returns the kind of the current session.
    pub fn kind(&self) -> SessionKind {
        self.kind
    }

    /// Returns the total configured duration for this timer.
    pub fn total_duration(&self) -> Duration {
        self.total_duration
    }

    /// Returns true if the timer is currently in a paused state.
    pub fn is_paused(&self) -> bool {
        self.paused_at.is_some()
    }

    /// Transitions the timer into a paused state.
    pub fn pause(&mut self, now: Instant) {
        if self.paused_at.is_none() {
            self.paused_at = Some(now);
        }
    }

    /// Transitions the timer out of a paused state and accumulates the elapsed pause time.
    pub fn resume(&mut self, now: Instant) {
        if let Some(paused_at) = self.paused_at.take() {
            self.paused_total += now.saturating_duration_since(paused_at);
        }
    }

    /// Toggles the pause state of the timer.
    pub fn toggle_pause(&mut self, now: Instant) {
        if self.is_paused() {
            self.resume(now);
        } else {
            self.pause(now);
        }
    }

    /// Calculates the total active (unpaused) time elapsed since start.
    pub fn elapsed_at(&self, now: Instant) -> Duration {
        let effective_now = self.paused_at.unwrap_or(now);
        effective_now
            .saturating_duration_since(self.started_at)
            .saturating_sub(self.paused_total)
    }

    /// Calculates the amount of time remaining until the timer reaches zero.
    pub fn remaining_at(&self, now: Instant) -> Duration {
        self.total_duration.saturating_sub(self.elapsed_at(now))
    }

    /// Calculates the completion percentage from 0.0 to 1.0.
    pub fn progress_at(&self, now: Instant) -> f32 {
        if self.total_duration.is_zero() {
            1.0
        } else {
            self.elapsed_at(now).as_secs_f32() / self.total_duration.as_secs_f32()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_defaults_to_twenty_five_minutes() {
        let started_at = Instant::now();
        let timer = ActiveTimer::new(SessionKind::Focus, None, started_at);

        assert_eq!(timer.total_duration(), Duration::from_secs(25 * 60));
    }

    #[test]
    fn break_defaults_to_five_minutes() {
        let started_at = Instant::now();
        let timer = ActiveTimer::new(SessionKind::Break, None, started_at);

        assert_eq!(timer.total_duration(), Duration::from_secs(5 * 60));
    }

    #[test]
    fn explicit_duration_configures_timer() {
        let started_at = Instant::now();
        let timer = ActiveTimer::new(
            SessionKind::Focus,
            Some(Duration::from_secs(90)),
            started_at,
        );

        assert_eq!(timer.total_duration(), Duration::from_secs(90));
    }

    #[test]
    fn pause_freezes_elapsed_time_until_resume() {
        let started_at = Instant::now();
        let mut timer = ActiveTimer::new(
            SessionKind::Focus,
            Some(Duration::from_secs(60)),
            started_at,
        );

        timer.pause(started_at + Duration::from_secs(10));

        assert_eq!(
            timer.elapsed_at(started_at + Duration::from_secs(30)),
            Duration::from_secs(10)
        );

        timer.resume(started_at + Duration::from_secs(40));

        assert_eq!(
            timer.elapsed_at(started_at + Duration::from_secs(50)),
            Duration::from_secs(20)
        );
    }

    #[test]
    fn remaining_time_saturates_at_zero() {
        let started_at = Instant::now();
        let timer = ActiveTimer::new(
            SessionKind::Focus,
            Some(Duration::from_secs(10)),
            started_at,
        );

        assert_eq!(
            timer.remaining_at(started_at + Duration::from_secs(20)),
            Duration::ZERO
        );
    }
}
