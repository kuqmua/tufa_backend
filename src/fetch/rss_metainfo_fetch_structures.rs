#[derive(Debug, Clone)] //Debug only for prints
pub enum HandledFetchStatusInfo {
    ResToTextError(String),
    ResStatusError(reqwest::StatusCode),
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

#[derive(Debug)] //Debug only for prints
pub enum RssFetchLinkError {
    ReqwestBlockingGet(reqwest::Error),
    StatusCode(reqwest::StatusCode),
    ResToTextError(reqwest::Error),
}

impl From<reqwest::Error> for RssFetchLinkError {
    fn from(e: reqwest::Error) -> Self {
        RssFetchLinkError::ReqwestBlockingGet(e)
    }
}

// impl Clone for RssFetchLinkError {
//     // fn clone(e: RssFetchLinkError) -> Self {
//     //     match e {
//     //         ReqwestBlockingGet(e) => RssFetchLinkError::ReqwestBlockingGet(e),
//     //         StatusCode(e) => RssFetchLinkError::StatusCode(e),
//     //         ResToTextError(e) => RssFetchLinkError::ResToTextError(e),
//     //     }
//     // }

//     fn clone_from(
//         self: &mut fetch::rss_metainfo_fetch_structures::RssFetchLinkError,
//         source: &Self,
//     ) {
//         match self {
//             Self::ReqwestBlockingGet(arg0) => Self::ReqwestBlockingGet(arg0),
//             Self::StatusCode(arg0) => Self::StatusCode(arg0),
//             Self::ResToTextError(arg0) => Self::ResToTextError(arg0),
//         };
//     }

//     fn clone(&self) -> Self {
//         match self {
//             Self::ReqwestBlockingGet(arg0) => Self::ReqwestBlockingGet(arg0),
//             Self::StatusCode(arg0) => Self::StatusCode(arg0),
//             Self::ResToTextError(arg0) => Self::ResToTextError(arg0),
//         }
//     }
// }
