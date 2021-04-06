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

pub fn handle_error_status_code(error_status_code: StatusCode, key: &str, url: String) -> bool {
    // println!(" handle_error_status_code {}", error_status_code)
    if error_status_code == reqwest::StatusCode::CONTINUE {
        println!("status 100 for key: {} url: {}", key, url); //Continue
    }
    if error_status_code == reqwest::StatusCode::SWITCHING_PROTOCOLS {
        println!("status 101 for key: {} url: {}", key, url); //Switching Protocols
    }
    if error_status_code == reqwest::StatusCode::PROCESSING {
        println!("status 102 for key: {} url: {}", key, url); //Processing
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 103 for key: {} url: {}", key, url); //Early Hints («ранняя метаинформация», key, url);
    // }
    if error_status_code == reqwest::StatusCode::OK {
        println!("status 200 for key: {} url: {}", key, url); // Success
    }
    if error_status_code == reqwest::StatusCode::CREATED {
        println!("status 201 for key: {} url: {}", key, url); //Created
    }
    if error_status_code == reqwest::StatusCode::ACCEPTED {
        println!("status 202 for key: {} url: {}", key, url); //Accepted
    }
    if error_status_code == reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION {
        println!("status 203 for key: {} url: {}", key, url); //Non-Authoritative Information
    }
    if error_status_code == reqwest::StatusCode::NO_CONTENT {
        println!("status 204 for key: {} url: {}", key, url); //No Content
    }
    if error_status_code == reqwest::StatusCode::RESET_CONTENT {
        println!("status 205 for key: {} url: {}", key, url); //Reset Content
    }
    if error_status_code == reqwest::StatusCode::PARTIAL_CONTENT {
        println!("status 206 for key: {} url: {}", key, url); //Partial Content
    }
    if error_status_code == reqwest::StatusCode::MULTI_STATUS {
        println!("status 207 for key: {} url: {}", key, url); //Multi-Status
    }
    if error_status_code == reqwest::StatusCode::ALREADY_REPORTED {
        println!("status 208 for key: {} url: {}", key, url); //Already Reported
    }
    if error_status_code == reqwest::StatusCode::IM_USED {
        println!("status 226 for key: {} url: {}", key, url); //IM Used
    }
    //Redirection
    if error_status_code == reqwest::StatusCode::MULTIPLE_CHOICES {
        println!("status 300 for key: {} url: {}", key, url); //Multiple Choices
    }
    if error_status_code == reqwest::StatusCode::MOVED_PERMANENTLY {
        println!("status 301 for key: {} url: {}", key, url); //Moved Permanently
    }
    if error_status_code == reqwest::StatusCode::FOUND {
        println!("status 302 for key: {} url: {}", key, url); //Moved Temporarily
    }
    // if error_status_code == 302 {//2 302 кода чего ????
    //     println!("status 302 for key: {} url: {}", key, url);//Found
    // }
    if error_status_code == reqwest::StatusCode::SEE_OTHER {
        println!("status 303 for key: {} url: {}", key, url); //See Other
    }
    if error_status_code == reqwest::StatusCode::NOT_MODIFIED {
        println!("status 304 for key: {} url: {}", key, url); //Not Modified
    }
    if error_status_code == reqwest::StatusCode::USE_PROXY {
        println!("status 305 for key: {} url: {}", key, url); //Use Proxy
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 306 for key: {} url: {}", key, url); //— зарезервировано (код использовался только в ранних спецификациях)
    // }
    if error_status_code == reqwest::StatusCode::TEMPORARY_REDIRECT {
        println!("status 307 for key: {} url: {}", key, url); //Temporary Redirect
    }
    if error_status_code == reqwest::StatusCode::PERMANENT_REDIRECT {
        println!("status 308 for key: {} url: {}", key, url); //Permanent Redirect
    }
    // 4xx: Client Error (ошибка клиента):
    if error_status_code == reqwest::StatusCode::BAD_REQUEST {
        println!("status 400 for key: {} url: {}", key, url); //Bad Request
    }
    if error_status_code == reqwest::StatusCode::UNAUTHORIZED {
        println!("status 401 for key: {} url: {}", key, url); //Unauthorized
    }
    if error_status_code == reqwest::StatusCode::PAYMENT_REQUIRED {
        println!("status 402 for key: {} url: {}", key, url); //Payment Required
    }
    if error_status_code == reqwest::StatusCode::FORBIDDEN {
        println!("status 403 for key: {} url: {}", key, url); //Forbidden
    }
    if error_status_code == reqwest::StatusCode::NOT_FOUND {
        println!("status 404 for key: {} url: {}", key, url); //Not Found
    }
    if error_status_code == reqwest::StatusCode::METHOD_NOT_ALLOWED {
        println!("status 405 for key: {} url: {}", key, url); //Method Not Allowed
    }
    if error_status_code == reqwest::StatusCode::NOT_ACCEPTABLE {
        println!("status 406 for key: {} url: {}", key, url); //Not Acceptable
    }
    if error_status_code == reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED {
        println!("status 407 for key: {} url: {}", key, url); //Proxy Authentication Required
    }
    if error_status_code == reqwest::StatusCode::REQUEST_TIMEOUT {
        println!("status 408 for key: {} url: {}", key, url); //Request Timeout
    }
    if error_status_code == reqwest::StatusCode::CONFLICT {
        println!("status 409 for key: {} url: {}", key, url); //Conflict
    }
    if error_status_code == reqwest::StatusCode::GONE {
        println!("status 410 for key: {} url: {}", key, url); //Gone
    }
    if error_status_code == reqwest::StatusCode::LENGTH_REQUIRED {
        println!("status 411 for key: {} url: {}", key, url); //Length Required
    }
    if error_status_code == reqwest::StatusCode::PRECONDITION_FAILED {
        println!("status 412 for key: {} url: {}", key, url); //Precondition Failed
    }
    if error_status_code == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
        println!("status 413 for key: {} url: {}", key, url); //Payload Too Large
    }
    if error_status_code == reqwest::StatusCode::URI_TOO_LONG {
        println!("status 414 for key: {} url: {}", key, url); //URI Too Long
    }
    if error_status_code == reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE {
        println!("status 415 for key: {} url: {}", key, url); //Unsupported Media Type
    }
    if error_status_code == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
        println!("status 416 for key: {} url: {}", key, url); //Range Not Satisfiable
    }
    if error_status_code == reqwest::StatusCode::EXPECTATION_FAILED {
        println!("status 417 for key: {} url: {}", key, url); //Expectation Failed
    }
    if error_status_code == reqwest::StatusCode::IM_A_TEAPOT {
        //что это за херня???????
        println!("status 418 for key: {} url: {}", key, url); //I’m a teapot
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 419 for key: {} url: {}", key, url); //Authentication Timeout
    // }
    //да, нет 420 хз поч
    if error_status_code == reqwest::StatusCode::MISDIRECTED_REQUEST {
        println!("status 421 for key: {} url: {}", key, url); //Misdirected Request
    }
    if error_status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        println!("status 422 for key: {} url: {}", key, url); //Unprocessable Entity
    }
    if error_status_code == reqwest::StatusCode::LOCKED {
        println!("status 423 for key: {} url: {}", key, url); //Locked
    }
    if error_status_code == reqwest::StatusCode::FAILED_DEPENDENCY {
        println!("status 424 for key: {} url: {}", key, url); //Failed Dependency
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 425 for key: {} url: {}", key, url); //Too Early
    // }
    if error_status_code == reqwest::StatusCode::UPGRADE_REQUIRED {
        println!("status 426 for key: {} url: {}", key, url); //Upgrade Required
    }
    //да, нет 427 хз поч
    if error_status_code == reqwest::StatusCode::PRECONDITION_REQUIRED {
        println!("status 428 for key: {} url: {}", key, url); //Precondition Required
    }
    if error_status_code == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!("status 429 for key: {} url: {}", key, url); //Too Many Requests
    }
    //да, нет 430 хз поч
    if error_status_code == reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE {
        println!("status 431 for key: {} url: {}", key, url); //Request Header Fields Too Large
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 449 for key: {} url: {}", key, url); //Retry With
    // }
    if error_status_code == reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS {
        println!("status 451 for key: {} url: {}", key, url); //Unavailable For Legal Reasons
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 499 for key: {} url: {}", key, url); //Client Closed Request
    // }
    // 5xx: Server Error (ошибка сервера):
    if error_status_code == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
        println!("status 500 for key: {} url: {}", key, url); //Internal Server Error
    }
    if error_status_code == reqwest::StatusCode::NOT_IMPLEMENTED {
        println!("status 501 for key: {} url: {}", key, url); //Not Implemented
    }
    if error_status_code == reqwest::StatusCode::BAD_GATEWAY {
        println!("status 502 for key: {} url: {}", key, url); //Bad Gateway
    }
    if error_status_code == reqwest::StatusCode::SERVICE_UNAVAILABLE {
        println!("status 503 for key: {} url: {}", key, url); //Service Unavailable
    }
    if error_status_code == reqwest::StatusCode::GATEWAY_TIMEOUT {
        println!("status 504 for key: {} url: {}", key, url); //Gateway Timeout
    }
    if error_status_code == reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED {
        println!("status 505 for key: {} url: {}", key, url); //HTTP Version Not Supported
    }
    if error_status_code == reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES {
        println!("status 506 for key: {} url: {}", key, url); //Variant Also Negotiates
    }
    if error_status_code == reqwest::StatusCode::INSUFFICIENT_STORAGE {
        println!("status 507 for key: {} url: {}", key, url); //Insufficient Storage
    }
    if error_status_code == reqwest::StatusCode::LOOP_DETECTED {
        println!("status 508 for key: {} url: {}", key, url); //Loop Detected
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("status 509 for key: {} url: {}", key, url); //Bandwidth Limit Exceeded
    // }
    if error_status_code == reqwest::StatusCode::NOT_EXTENDED {
        println!("status 510 for key: {} url: {}", key, url); //Not Extended
    }
    if error_status_code == reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED {
        println!("status 511 for key: {} url: {}", key, url); //Network Authentication Required
    }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 520 for key: {} url: {}", key, url); //Unknown Error
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 521 for key: {} url: {}", key, url); //Web Server Is Down
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 522 for key: {} url: {}", key, url); //Connection Timed Out
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 523 for key: {} url: {}", key, url); //Origin Is Unreachable
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 524 for key: {} url: {}", key, url); //A Timeout Occurred
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 525 for key: {} url: {}", key, url); //SSL Handshake Failed
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("status 526 for key: {} url: {}", key, url); //Invalid SSL Certificate
    // }
    false
}
