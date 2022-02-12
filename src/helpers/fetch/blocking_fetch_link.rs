use crate::helpers::fetch::fetch_link_error::FetchLinkError;
use crate::helpers::fetch::fetch_link_error::FetchLinkErrorEnum;

use crate::helpers::where_was::WhereWas;

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn blocking_fetch_link(link: &str) -> Result<String, FetchLinkError> {
    match reqwest::blocking::get(link) {
        Err(e) => Err(FetchLinkError {
            source: Box::new(FetchLinkErrorEnum::ReqwestBlockingGet {
                source: e,
                where_was: WhereWas {
                    file: file!(),
                    line: line!(),
                    column: column!(),
                },
            }),
        }),
        Ok(res) => {
            let status = res.status();
            if status != reqwest::StatusCode::OK {
                return Err(FetchLinkError {
                    source: Box::new(FetchLinkErrorEnum::StatusCode {
                        source: status,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                });
            }
            match res.text() {
                Err(e) => Err(FetchLinkError {
                    source: Box::new(FetchLinkErrorEnum::ResponseText {
                        source: e,
                        where_was: WhereWas {
                            file: file!(),
                            line: line!(),
                            column: column!(),
                        },
                    }),
                }),
                Ok(text) => Ok(text),
            }
        }
    }
}
