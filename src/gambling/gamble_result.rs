#[derive(Debug, PartialEq,Eq,Copy, Clone)]
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
        *self==GambleResult::Success
    }
    pub fn is_failure(&self) -> bool {
        *self==GambleResult::Failure
    }
    pub fn is_partial_success(&self) -> bool {
        *self == GambleResult::PartialSuccess
    }
}
