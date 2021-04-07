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
        println!("status 100 for key: {} link: {}", key, link); //Continue
    }
    if error_status_code == reqwest::StatusCode::SWITCHING_PROTOCOLS {
        println!("status 101 for key: {} link: {}", key, link); //Switching Protocols
    }
    if error_status_code == reqwest::StatusCode::PROCESSING {
        println!("status 102 for key: {} link: {}", key, link); //Processing
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 103 for key: {} link: {}", key, link); //Early Hints («ранняя метаинформация», key, link);
    // }
    if error_status_code == reqwest::StatusCode::OK {
        println!("status 200 for key: {} link: {}", key, link); // Success
    }
    if error_status_code == reqwest::StatusCode::CREATED {
        println!("status 201 for key: {} link: {}", key, link); //Created
    }
    if error_status_code == reqwest::StatusCode::ACCEPTED {
        println!("status 202 for key: {} link: {}", key, link); //Accepted
    }
    if error_status_code == reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION {
        println!("status 203 for key: {} link: {}", key, link); //Non-Authoritative Information
    }
    if error_status_code == reqwest::StatusCode::NO_CONTENT {
        println!("status 204 for key: {} link: {}", key, link); //No Content
    }
    if error_status_code == reqwest::StatusCode::RESET_CONTENT {
        println!("status 205 for key: {} link: {}", key, link); //Reset Content
    }
    if error_status_code == reqwest::StatusCode::PARTIAL_CONTENT {
        println!("status 206 for key: {} link: {}", key, link); //Partial Content
    }
    if error_status_code == reqwest::StatusCode::MULTI_STATUS {
        println!("status 207 for key: {} link: {}", key, link); //Multi-Status
    }
    if error_status_code == reqwest::StatusCode::ALREADY_REPORTED {
        println!("status 208 for key: {} link: {}", key, link); //Already Reported
    }
    if error_status_code == reqwest::StatusCode::IM_USED {
        println!("status 226 for key: {} link: {}", key, link); //IM Used
    }
    //Redirection
    if error_status_code == reqwest::StatusCode::MULTIPLE_CHOICES {
        println!("status 300 for key: {} link: {}", key, link); //Multiple Choices
    }
    if error_status_code == reqwest::StatusCode::MOVED_PERMANENTLY {
        println!("status 301 for key: {} link: {}", key, link); //Moved Permanently
    }
    if error_status_code == reqwest::StatusCode::FOUND {
        println!("status 302 for key: {} link: {}", key, link); //Moved Temporarily
    }
    // if error_status_code == 302 {//2 302 кода чего ????
    //     println!("status 302 for key: {} link: {}", key, link);//Found
    // }
    if error_status_code == reqwest::StatusCode::SEE_OTHER {
        println!("status 303 for key: {} link: {}", key, link); //See Other
    }
    if error_status_code == reqwest::StatusCode::NOT_MODIFIED {
        println!("status 304 for key: {} link: {}", key, link); //Not Modified
    }
    if error_status_code == reqwest::StatusCode::USE_PROXY {
        println!("status 305 for key: {} link: {}", key, link); //Use Proxy
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 306 for key: {} link: {}", key, link); //— зарезервировано (код использовался только в ранних спецификациях)
    // }
    if error_status_code == reqwest::StatusCode::TEMPORARY_REDIRECT {
        println!("status 307 for key: {} link: {}", key, link); //Temporary Redirect
    }
    if error_status_code == reqwest::StatusCode::PERMANENT_REDIRECT {
        println!("status 308 for key: {} link: {}", key, link); //Permanent Redirect
    }
    // 4xx: Client Error (ошибка клиента):
    if error_status_code == reqwest::StatusCode::BAD_REQUEST {
        println!("status 400 for key: {} link: {}", key, link); //Bad Request
    }
    if error_status_code == reqwest::StatusCode::UNAUTHORIZED {
        println!("status 401 for key: {} link: {}", key, link); //Unauthorized
    }
    if error_status_code == reqwest::StatusCode::PAYMENT_REQUIRED {
        println!("status 402 for key: {} link: {}", key, link); //Payment Required
    }
    if error_status_code == reqwest::StatusCode::FORBIDDEN {
        println!("status 403 for key: {} link: {}", key, link); //Forbidden
    }
    if error_status_code == reqwest::StatusCode::NOT_FOUND {
        println!("status 404 for key: {} link: {}", key, link); //Not Found
    }
    if error_status_code == reqwest::StatusCode::METHOD_NOT_ALLOWED {
        println!("status 405 for key: {} link: {}", key, link); //Method Not Allowed
    }
    if error_status_code == reqwest::StatusCode::NOT_ACCEPTABLE {
        println!("status 406 for key: {} link: {}", key, link); //Not Acceptable
    }
    if error_status_code == reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED {
        println!("status 407 for key: {} link: {}", key, link); //Proxy Authentication Required
    }
    if error_status_code == reqwest::StatusCode::REQUEST_TIMEOUT {
        println!("status 408 for key: {} link: {}", key, link); //Request Timeout
    }
    if error_status_code == reqwest::StatusCode::CONFLICT {
        println!("status 409 for key: {} link: {}", key, link); //Conflict
    }
    if error_status_code == reqwest::StatusCode::GONE {
        println!("status 410 for key: {} link: {}", key, link); //Gone
    }
    if error_status_code == reqwest::StatusCode::LENGTH_REQUIRED {
        println!("status 411 for key: {} link: {}", key, link); //Length Required
    }
    if error_status_code == reqwest::StatusCode::PRECONDITION_FAILED {
        println!("status 412 for key: {} link: {}", key, link); //Precondition Failed
    }
    if error_status_code == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
        println!("status 413 for key: {} link: {}", key, link); //Payload Too Large
    }
    if error_status_code == reqwest::StatusCode::URI_TOO_LONG {
        println!("status 414 for key: {} link: {}", key, link); //URI Too Long
    }
    if error_status_code == reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE {
        println!("status 415 for key: {} link: {}", key, link); //Unsupported Media Type
    }
    if error_status_code == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
        println!("status 416 for key: {} link: {}", key, link); //Range Not Satisfiable
    }
    if error_status_code == reqwest::StatusCode::EXPECTATION_FAILED {
        println!("status 417 for key: {} link: {}", key, link); //Expectation Failed
    }
    if error_status_code == reqwest::StatusCode::IM_A_TEAPOT {
        //что это за херня???????
        println!("status 418 for key: {} link: {}", key, link); //I’m a teapot
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 419 for key: {} link: {}", key, link); //Authentication Timeout
    // }
    //да, нет 420 хз поч
    if error_status_code == reqwest::StatusCode::MISDIRECTED_REQUEST {
        println!("status 421 for key: {} link: {}", key, link); //Misdirected Request
    }
    if error_status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        println!("status 422 for key: {} link: {}", key, link); //Unprocessable Entity
    }
    if error_status_code == reqwest::StatusCode::LOCKED {
        println!("status 423 for key: {} link: {}", key, link); //Locked
    }
    if error_status_code == reqwest::StatusCode::FAILED_DEPENDENCY {
        println!("status 424 for key: {} link: {}", key, link); //Failed Dependency
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 425 for key: {} link: {}", key, link); //Too Early
    // }
    if error_status_code == reqwest::StatusCode::UPGRADE_REQUIRED {
        println!("status 426 for key: {} link: {}", key, link); //Upgrade Required
    }
    //да, нет 427 хз поч
    if error_status_code == reqwest::StatusCode::PRECONDITION_REQUIRED {
        println!("status 428 for key: {} link: {}", key, link); //Precondition Required
    }
    if error_status_code == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!("status 429 for key: {} link: {}", key, link); //Too Many Requests
    }
    //да, нет 430 хз поч
    if error_status_code == reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE {
        println!("status 431 for key: {} link: {}", key, link); //Request Header Fields Too Large
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 449 for key: {} link: {}", key, link); //Retry With
    // }
    if error_status_code == reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS {
        println!("status 451 for key: {} link: {}", key, link); //Unavailable For Legal Reasons
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 499 for key: {} link: {}", key, link); //Client Closed Request
    // }
    // 5xx: Server Error (ошибка сервера):
    if error_status_code == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
        println!("status 500 for key: {} link: {}", key, link); //Internal Server Error
    }
    if error_status_code == reqwest::StatusCode::NOT_IMPLEMENTED {
        println!("status 501 for key: {} link: {}", key, link); //Not Implemented
    }
    if error_status_code == reqwest::StatusCode::BAD_GATEWAY {
        println!("status 502 for key: {} link: {}", key, link); //Bad Gateway
    }
    if error_status_code == reqwest::StatusCode::SERVICE_UNAVAILABLE {
        println!("status 503 for key: {} link: {}", key, link); //Service Unavailable
    }
    if error_status_code == reqwest::StatusCode::GATEWAY_TIMEOUT {
        println!("status 504 for key: {} link: {}", key, link); //Gateway Timeout
    }
    if error_status_code == reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED {
        println!("status 505 for key: {} link: {}", key, link); //HTTP Version Not Supported
    }
    if error_status_code == reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES {
        println!("status 506 for key: {} link: {}", key, link); //Variant Also Negotiates
    }
    if error_status_code == reqwest::StatusCode::INSUFFICIENT_STORAGE {
        println!("status 507 for key: {} link: {}", key, link); //Insufficient Storage
    }
    if error_status_code == reqwest::StatusCode::LOOP_DETECTED {
        println!("status 508 for key: {} link: {}", key, link); //Loop Detected
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 509 for key: {} link: {}", key, link); //Bandwidth Limit Exceeded
    // }
    if error_status_code == reqwest::StatusCode::NOT_EXTENDED {
        println!("status 510 for key: {} link: {}", key, link); //Not Extended
    }
    if error_status_code == reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED {
        println!("status 511 for key: {} link: {}", key, link); //Network Authentication Required
    }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 520 for key: {} link: {}", key, link); //Unknown Error
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 521 for key: {} link: {}", key, link); //Web Server Is Down
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 522 for key: {} link: {}", key, link); //Connection Timed Out
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 523 for key: {} link: {}", key, link); //Origin Is Unreachable
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 524 for key: {} link: {}", key, link); //A Timeout Occurred
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 525 for key: {} link: {}", key, link); //SSL Handshake Failed
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 526 for key: {} link: {}", key, link); //Invalid SSL Certificate
    // }
    false
}
