use tufa_common::where_was::WhereWas;

#[derive(Debug)]
pub enum HttpRequestTextErrorEnum {
    ReqwestBlockingGet {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    StatusCode {
        source: reqwest::Error,
        where_was: WhereWas,
    },
    ResponseText {
        source: reqwest::Error,
        where_was: WhereWas,
    },
}
