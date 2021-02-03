pub enum HandledReachProviderStatusInfo {
    Initialized,
    ResStatusError(reqwest::StatusCode),
    Success,
}

pub enum UnhandledReachProviderInfo {
    Failure(String),
    Success,
}
