use super::check_link_metainfo_structures::HandledReachProviderStatusInfo;
use super::check_link_metainfo_structures::UnhandledReachProviderInfo;
use super::fetch_link::fetch_link;
use crate::config::ENABLE_ERROR_PRINTS_HANDLE;
use crate::overriding::prints::print_error_red;

pub fn check_link(
    url: &str,
) -> (
    bool,
    UnhandledReachProviderInfo,
    HandledReachProviderStatusInfo,
) {
    let fetch_result = fetch_link(url);
    let mut can_i: bool = false;
    let unhandled_info: UnhandledReachProviderInfo;
    let handled_info: HandledReachProviderStatusInfo;
    match fetch_result {
        Ok(fetch_tuple_result) => {
            if fetch_tuple_result.0 {
                can_i = true;
                unhandled_info = UnhandledReachProviderInfo::Success;
                handled_info = fetch_tuple_result.1;
            } else {
                if ENABLE_ERROR_PRINTS_HANDLE {
                    match fetch_tuple_result.1 {
                        HandledReachProviderStatusInfo::ResStatusError(status_code) => {
                            print_error_red(
                                file!().to_string(),
                                line!().to_string(),
                                status_code.clone().to_string(),
                            )
                        }
                        HandledReachProviderStatusInfo::Initialized => print_error_red(
                            file!().to_string(),
                            line!().to_string(),
                            "check_link false, and HandledReachProviderStatusInfo::Initialized??? wtf"
                                .to_string(),
                        ),
                        HandledReachProviderStatusInfo::Success => print_error_red(
                            file!().to_string(),
                            line!().to_string(),
                            "check_link false, and HandledReachProviderStatusInfo::Success??? wtf"
                                .to_string(),
                        ),
                    }
                }
                can_i = false;
                unhandled_info = UnhandledReachProviderInfo::Success;
                handled_info = fetch_tuple_result.1;
            }
        }
        Err(e) => {
            unhandled_info = UnhandledReachProviderInfo::Failure(e.to_string());
            handled_info = HandledReachProviderStatusInfo::Initialized;
            if ENABLE_ERROR_PRINTS_HANDLE {
                let error = "check_link fetch_result Box<dyn Error>".to_string() + &e.to_string();
                print_error_red(file!().to_string(), line!().to_string(), error)
            }
        }
    }
    (can_i, unhandled_info, handled_info)
}
