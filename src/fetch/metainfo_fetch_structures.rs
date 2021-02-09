pub enum HandledFetchStatusInfo {
    Initialized,
    ResToTextError(String),
    ResStatusError(reqwest::StatusCode),
    Success,
}
pub enum UnhandledFetchStatusInfo {
    Failure(String),
    Initialized,
    Success,
}
pub enum AreThereItems {
    Yep,
    Initialized,
    NopeButThereIsTag(String),
    ConversionFromStrError(String, String),
    NopeNoTag(String),
}
