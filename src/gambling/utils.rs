use std::time::Instant;

#[derive(Debug)]
pub struct LastChange {
    pub inner: Instant,
}

impl Default for LastChange {
    fn default() -> LastChange {
        LastChange {
            inner: Instant::now(),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum AnimStatus {
    #[default]
    Unitialized,
    Ongoing,
    Finished,
}

impl AnimStatus {
    pub fn is_uninit(&self) -> bool {
        *self == AnimStatus::Unitialized
    }
    pub fn is_ongoing(&self) -> bool {
        *self == AnimStatus::Ongoing
    }
    pub fn is_finished(&self) -> bool {
        *self == AnimStatus::Finished
    }
    pub fn start(&mut self) {
        if self.is_uninit() {
            *self = AnimStatus::Ongoing;
        }
    }
    pub fn finish(&mut self) {
        //stopped also to avoid conflicts
        if self.is_ongoing() {
            *self = AnimStatus::Finished;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GambleResult {
    Success,
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
    pub fn is_failure(&self) -> bool {
        *self == GambleResult::Failure
    }
}
