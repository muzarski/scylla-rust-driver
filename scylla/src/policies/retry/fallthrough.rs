use super::{RequestInfo, RetryDecision, RetryPolicy, RetrySession};

/// Forwards all errors directly to the user, never retries
#[derive(Debug)]
pub struct FallthroughRetryPolicy;
pub struct FallthroughRetrySession;

impl FallthroughRetryPolicy {
    pub fn new() -> FallthroughRetryPolicy {
        FallthroughRetryPolicy
    }
}

impl Default for FallthroughRetryPolicy {
    fn default() -> FallthroughRetryPolicy {
        FallthroughRetryPolicy
    }
}

impl RetryPolicy for FallthroughRetryPolicy {
    fn new_session(&self) -> Box<dyn RetrySession> {
        Box::new(FallthroughRetrySession)
    }
}

impl RetrySession for FallthroughRetrySession {
    fn decide_should_retry(&mut self, _query_info: RequestInfo) -> RetryDecision {
        RetryDecision::DontRetry
    }

    fn reset(&mut self) {}
}
