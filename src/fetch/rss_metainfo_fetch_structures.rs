#[derive(Debug, Clone)] //Debug only for prints
pub enum HandledFetchStatusInfo {
    ResToTextError(String),
    ResStatusError(reqwest::StatusCode),
    Success,
}
#[derive(Debug, Clone)] //Debug only for prints
pub enum UnhandledFetchStatusInfo {
    Failure(String),
    Success,
}
#[derive(Debug, Clone)] //Debug only for prints
pub enum AreThereItems {
    Yep,
    NopeButThereIsTag(String),
    ConversionFromStrError(String, String),
    NopeNoTag(String),
}
