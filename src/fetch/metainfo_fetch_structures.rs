#[derive(Debug)] //only for prints
pub enum HandledFetchStatusInfo {
    Initialized,
    ResToTextError(String),
    ResStatusError(reqwest::StatusCode),
    Success,
}
#[derive(Debug)] //only for prints
pub enum UnhandledFetchStatusInfo {
    Failure(String),
    Initialized,
    Success,
}
#[derive(Debug)] //only for prints
pub enum AreThereItems {
    Yep,
    Initialized,
    NopeButThereIsTag(String),
    ConversionFromStrError(String, String),
    NopeNoTag(String),
}
