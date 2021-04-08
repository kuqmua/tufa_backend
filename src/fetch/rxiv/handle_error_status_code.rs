// extern crate reqwest;
// extern crate serde;
// extern crate serde_xml_rs;

// use crate::check_net::check_link::check_link;
// use crate::fetch::metainfo_fetch_structures::AreThereItems;
// use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
// use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
// use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
// use crate::fetch::provider_kind_enum::RxivKind;
// use crate::fetch::rxiv_structures::RxivPostStruct;
// use std::collections::HashMap;
use reqwest::StatusCode;

pub fn handle_error_status_code(error_status_code: StatusCode, key: &str, link: String) -> bool {
    // println!(" handle_error_status_code {}", error_status_code)
    if error_status_code == reqwest::StatusCode::CONTINUE {
        println!("status 100(Continue) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::SWITCHING_PROTOCOLS {
        println!(
            "status 101(Switching Protocols) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::PROCESSING {
        println!("status 102(Processing) for key: {} link: {}", key, link);
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 103(Early Hints) for key: {} link: {}", key, link); //Early Hints («ранняя метаинформация», key, link);
    // }
    if error_status_code == reqwest::StatusCode::OK {
        println!("status 200(Success) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::CREATED {
        println!("status 201(Created) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::ACCEPTED {
        println!("status 202(Accepted) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION {
        println!(
            "status 203(Non-Authoritative Information) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::NO_CONTENT {
        println!("status 204(No Content) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::RESET_CONTENT {
        println!("status 205(Reset Content) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::PARTIAL_CONTENT {
        println!(
            "status 206(Partial Content) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::MULTI_STATUS {
        println!("status 207(Multi-Status) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::ALREADY_REPORTED {
        println!(
            "status 208(Already Reported) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::IM_USED {
        println!("status 226(IM Used) for key: {} link: {}", key, link);
    }
    //Redirection
    if error_status_code == reqwest::StatusCode::MULTIPLE_CHOICES {
        println!(
            "status 300(Multiple Choices) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::MOVED_PERMANENTLY {
        println!(
            "status 301(Moved Permanently) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::FOUND {
        println!(
            "status 302(Moved Temporarily) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::SEE_OTHER {
        println!("status 303(See Other) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::NOT_MODIFIED {
        println!("status 304(Not Modified) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::USE_PROXY {
        println!("status 305(Use Proxy) for key: {} link: {}", key, link);
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 306() for key: {} link: {}", key, link); //— зарезервировано (код использовался только в ранних спецификациях)
    // }
    if error_status_code == reqwest::StatusCode::TEMPORARY_REDIRECT {
        println!(
            "status 307(Temporary Redirect) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::PERMANENT_REDIRECT {
        println!(
            "status 308(Permanent Redirect) for key: {} link: {}",
            key, link
        );
    }
    // 4xx: Client Error (ошибка клиента):
    if error_status_code == reqwest::StatusCode::BAD_REQUEST {
        println!("status 400(Bad Request) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::UNAUTHORIZED {
        println!("status 401(Unauthorized) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::PAYMENT_REQUIRED {
        println!(
            "status 402(Payment Required) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::FORBIDDEN {
        println!("status 403(Forbidden) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::NOT_FOUND {
        println!("status 404(Not Found) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::METHOD_NOT_ALLOWED {
        println!(
            "status 405(Method Not Allowed) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::NOT_ACCEPTABLE {
        println!("status 406(Not Acceptable) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED {
        println!(
            "status 407(Proxy Authentication Required) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::REQUEST_TIMEOUT {
        println!(
            "status 408(Request Timeout) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::CONFLICT {
        println!("status 409(Conflict) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::GONE {
        println!("status 410(Gone) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::LENGTH_REQUIRED {
        println!(
            "status 411(Length Required) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::PRECONDITION_FAILED {
        println!(
            "status 412(Precondition Failed) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
        println!(
            "status 413(Payload Too Large) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::URI_TOO_LONG {
        println!("status 414(URI Too Long) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE {
        println!(
            "status 415(Unsupported Media Type) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
        println!(
            "status 416(Range Not Satisfiable) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::EXPECTATION_FAILED {
        println!(
            "status 417(Expectation Failed) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::IM_A_TEAPOT {
        //что это за херня???????
        println!("status 418(I’m a teapot) for key: {} link: {}", key, link); //I’m a teapot
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 419() for key: {} link: {}", key, link); //Authentication Timeout
    // }
    //да, нет 420 хз поч
    if error_status_code == reqwest::StatusCode::MISDIRECTED_REQUEST {
        println!(
            "status 421(Misdirected Request) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        println!(
            "status 422(Unprocessable Entity) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::LOCKED {
        println!("status 423(Locked) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::FAILED_DEPENDENCY {
        println!(
            "status 424(Failed Dependency) for key: {} link: {}",
            key, link
        );
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 425(Too Early) for key: {} link: {}", key, link);
    // }
    if error_status_code == reqwest::StatusCode::UPGRADE_REQUIRED {
        println!(
            "status 426(Upgrade Required) for key: {} link: {}",
            key, link
        );
    }
    //да, нет 427 хз поч
    if error_status_code == reqwest::StatusCode::PRECONDITION_REQUIRED {
        println!(
            "status 428(Precondition Required) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!(
            "status 429(Too Many Requests) for key: {} link: {}",
            key, link
        );
    }
    //да, нет 430 хз поч
    if error_status_code == reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE {
        println!(
            "status 431(Request Header Fields Too Large) for key: {} link: {}",
            key, link
        );
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 449(Retry With) for key: {} link: {}", key, link);
    // }
    if error_status_code == reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS {
        println!(
            "status 451(Unavailable For Legal Reasons) for key: {} link: {}",
            key, link
        );
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 499(Client Closed Request) for key: {} link: {}", key, link);
    // }
    // 5xx: Server Error (ошибка сервера):
    if error_status_code == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
        println!(
            "status 500(Internal Server Error) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::NOT_IMPLEMENTED {
        println!(
            "status 501(Not Implemented) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::BAD_GATEWAY {
        println!("status 502(Bad Gateway) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::SERVICE_UNAVAILABLE {
        println!(
            "status 503(Service Unavailable) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::GATEWAY_TIMEOUT {
        println!(
            "status 504(Gateway Timeout) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED {
        println!(
            "status 505(HTTP Version Not Supported) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES {
        println!(
            "status 506(Variant Also Negotiates) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::INSUFFICIENT_STORAGE {
        println!(
            "status 507(Insufficient Storage) for key: {} link: {}",
            key, link
        );
    }
    if error_status_code == reqwest::StatusCode::LOOP_DETECTED {
        println!("status 508(Loop Detected) for key: {} link: {}", key, link);
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 509(Bandwidth Limit Exceeded) for key: {} link: {}", key, link);
    // }
    if error_status_code == reqwest::StatusCode::NOT_EXTENDED {
        println!("status 510(Not Extended) for key: {} link: {}", key, link);
    }
    if error_status_code == reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED {
        println!(
            "status 511(Network Authentication Required) for key: {} link: {}",
            key, link
        );
    }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 520(Unknown Error) for key: {} link: {}", key, link);
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 521(Web Server Is Down) for key: {} link: {}", key, link);
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 522(Connection Timed Out) for key: {} link: {}", key, link);
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 523(Origin Is Unreachable) for key: {} link: {}", key, link);
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 524(A Timeout Occurred) for key: {} link: {}", key, link);
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 525(SSL Handshake Failed) for key: {} link: {}", key, link);
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 526(Invalid SSL Certificate) for key: {} link: {}", key, link);
    // }
    false
}
