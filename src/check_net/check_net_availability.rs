use std::fmt;

use crate::check_net::check_link_status_code::check_link_status_code;

use reqwest::StatusCode;

#[derive(thiserror::Error, displaydoc::Display, Debug)]
pub enum CheckNetAvailabilityError {
    ///CheckNetAvailabilityError: reqwest: {source:?}
    ReqwestError { source: Box<reqwest::Error> },
    ///CheckNetAvailabilityError: StatusCode: {source:?}
    StatusCode { source: StatusCodeError },
}

#[derive(Debug)] //Default,//serde_derive::Serialize, serde_derive::Deserialize
pub struct StatusCodeError {
    pub status_code: StatusCode,
}
// use std::error::Error;
impl std::error::Error for StatusCodeError {}

impl fmt::Display for StatusCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
// pub trait Error: Debug + Display {
//     fn description(&self) -> &str { /* ... */ }
//     fn cause(&self) -> Option<&Error> { /* ... */ }
//     fn source(&self) -> Option<&(Error + 'static)> { /* ... */ }
// }

// impl Error for StatusCode {
//     fn source(&self) -> Option<Self> {
//         Some(self)
//     }

//     // fn type_id(&self, _: private::Internal) -> std::any::TypeId
//     // where
//     //     Self: 'static,
//     // {
//     //     std::any::TypeId::of::<Self>()
//     // }

//     // fn backtrace(&self) -> Option<&std::backtrace::Backtrace> {
//     //     None
//     // }

//     fn description(&self) -> &str {
//         "description() is deprecated; use Display"
//     }

//     fn cause(&self) -> Option<&dyn std::error::Error> {
//         self.source()
//     }
//     // fn backtrace(&self) -> Option<&Backtrace> {
//     //     None
//     // }
//     // fn description(&self) -> &str {
//     //     None
//     // }
//     // fn cause(&self) -> Option<&dyn Error> {
//     //     None
//     // }
// }

#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
pub fn check_net_availability(link: &str) -> Result<(), Box<CheckNetAvailabilityError>> {
    match check_link_status_code(link) {
        Err(e) => Err(Box::new(CheckNetAvailabilityError::ReqwestError {
            source: e,
        })),
        Ok(status_code) => {
            if !StatusCode::is_success(&status_code) {
                return Err(Box::new(CheckNetAvailabilityError::StatusCode {
                    source: StatusCodeError { status_code },
                }));
            }
            Ok(())
        }
    }
}
