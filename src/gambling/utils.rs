use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum AnimStatus {
    #[default]
    Unitialized,
    Ongoing,
    Stopped,
    Finished,
}

impl AnimStatus {
    pub fn is_uninit(&self) -> bool {
        *self == AnimStatus::Unitialized
    }
    pub fn is_ongoing(&self) -> bool {
        *self == AnimStatus::Ongoing
    }
    pub fn is_stopped(&self) -> bool {
        *self == AnimStatus::Stopped
    }
    pub fn is_finished(&self) -> bool {
        *self == AnimStatus::Finished
    }
    pub fn start(&mut self) {
        if self.is_uninit() || self.is_stopped() {
            *self = AnimStatus::Ongoing;
        }
    }
    pub fn stop(&mut self) {
        if self.is_ongoing() {
            *self = AnimStatus::Stopped;
        }
    }
    pub fn finished(&mut self) {
        //stopped also to avoid conflicts
        if self.is_ongoing() || self.is_stopped() {
            *self = AnimStatus::Finished;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GambleResult {
    Success,
    PartialSuccess,
    Failure,
}

//developer ergonomics hehe
impl From<bool> for GambleResult {
    fn from(b: bool) -> Self {
        if b {
            return GambleResult::Success;
        }
        GambleResult::Failure
    }
}

impl GambleResult {
    pub fn is_success(&self) -> bool {
        *self == GambleResult::Success
    }
    pub fn is_failure(&self) -> bool {
        *self == GambleResult::Failure
    }
    pub fn is_partial_success(&self) -> bool {
        *self == GambleResult::PartialSuccess
    }
}
