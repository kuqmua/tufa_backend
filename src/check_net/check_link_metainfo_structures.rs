#[derive(Debug)]
pub enum HandledReachProviderStatusInfo {
    Initialized,
    ResStatusError(reqwest::StatusCode),
    Success,
}
#[derive(Debug)]
pub enum UnhandledReachProviderInfo {
    Failure(String),
    Success,
}
