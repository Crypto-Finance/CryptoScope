use std::time::Instant;

/// Duration in seconds before popup messages auto-dismiss.
const POPUP_AUTO_DISMISS_SECS: u64 = 5;

/// Manages popup message display and auto-dismiss timing.
#[derive(Debug, Default)]
pub struct PopupState {
    pub message: Option<String>,
    pub timer: Option<Instant>,
    pub is_error: bool,
}

impl PopupState {
    /// Show a popup message with the given error/info flag.
    pub fn show(&mut self, message: String, is_error: bool) {
        self.message = Some(message);
        self.timer = Some(Instant::now());
        self.is_error = is_error;
    }

    /// Check if the popup auto-dismiss timer has elapsed.
    pub fn update(&mut self) {
        if let Some(timer) = self.timer
            && timer.elapsed().as_secs() >= POPUP_AUTO_DISMISS_SECS
        {
            self.message = None;
            self.timer = None;
        }
    }

    /// Dismiss the current popup message immediately.
    pub fn dismiss(&mut self) {
        self.message = None;
        self.timer = None;
    }
}
