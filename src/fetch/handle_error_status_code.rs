// extern crate reqwest;
// extern crate serde;
// extern crate serde_xml_rs;

// use crate::check_net::check_link::check_link;
// use crate::fetch::metainfo_fetch_structures::AreThereItems;
// use crate::fetch::metainfo_fetch_structures::HandledFetchStatusInfo;
// use crate::fetch::metainfo_fetch_structures::UnhandledFetchStatusInfo;
// use crate::fetch::rxiv_fetch_and_parse_xml::rxiv_fetch_and_parse_xml;
// use crate::fetch::rxiv_kind_enum::RxivKind;
// use crate::fetch::rxiv_structures::RxivPostStruct;
// use std::collections::HashMap;
use reqwest::StatusCode;

pub fn handle_error_status_code(error_status_code: StatusCode) -> bool {
    // println!(" handle_error_status_code {}", error_status_code)
    if error_status_code == reqwest::StatusCode::CONTINUE {
        println!("100"); //Continue
    }
    if error_status_code == reqwest::StatusCode::SWITCHING_PROTOCOLS {
        println!("101"); //Switching Protocols
    }
    if error_status_code == reqwest::StatusCode::PROCESSING {
        println!("102"); //Processing
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("103"); //Early Hints («ранняя метаинформация»);
    // }
    if error_status_code == reqwest::StatusCode::OK {
        println!("200"); // Success
    }
    if error_status_code == reqwest::StatusCode::CREATED {
        println!("201"); //Created
    }
    if error_status_code == reqwest::StatusCode::ACCEPTED {
        println!("202"); //Accepted
    }
    if error_status_code == reqwest::StatusCode::NON_AUTHORITATIVE_INFORMATION {
        println!("203"); //Non-Authoritative Information
    }
    if error_status_code == reqwest::StatusCode::NO_CONTENT {
        println!("204"); //No Content
    }
    if error_status_code == reqwest::StatusCode::RESET_CONTENT {
        println!("205"); //Reset Content
    }
    if error_status_code == reqwest::StatusCode::PARTIAL_CONTENT {
        println!("206"); //Partial Content
    }
    if error_status_code == reqwest::StatusCode::MULTI_STATUS {
        println!("207"); //Multi-Status
    }
    if error_status_code == reqwest::StatusCode::ALREADY_REPORTED {
        println!("208"); //Already Reported
    }
    if error_status_code == reqwest::StatusCode::IM_USED {
        println!("226"); //IM Used
    }
    //Redirection
    if error_status_code == reqwest::StatusCode::MULTIPLE_CHOICES {
        println!("300"); //Multiple Choices
    }
    if error_status_code == reqwest::StatusCode::MOVED_PERMANENTLY {
        println!("301"); //Moved Permanently
    }
    if error_status_code == reqwest::StatusCode::FOUND {
        println!("302"); //Moved Temporarily
    }
    // if error_status_code == 302 {//2 302 кода чего ????
    //     println!("302");//Found
    // }
    if error_status_code == reqwest::StatusCode::SEE_OTHER {
        println!("303"); //See Other
    }
    if error_status_code == reqwest::StatusCode::NOT_MODIFIED {
        println!("304"); //Not Modified
    }
    if error_status_code == reqwest::StatusCode::USE_PROXY {
        println!("305"); //Use Proxy
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("306"); //— зарезервировано (код использовался только в ранних спецификациях)
    // }
    if error_status_code == reqwest::StatusCode::TEMPORARY_REDIRECT {
        println!("307"); //Temporary Redirect
    }
    if error_status_code == reqwest::StatusCode::PERMANENT_REDIRECT {
        println!("308"); //Permanent Redirect
    }
    // 4xx: Client Error (ошибка клиента):
    if error_status_code == reqwest::StatusCode::BAD_REQUEST {
        println!("400"); //Bad Request
    }
    if error_status_code == reqwest::StatusCode::UNAUTHORIZED {
        println!("401"); //Unauthorized
    }
    if error_status_code == reqwest::StatusCode::PAYMENT_REQUIRED {
        println!("402"); //Payment Required
    }
    if error_status_code == reqwest::StatusCode::FORBIDDEN {
        println!("403"); //Forbidden
    }
    if error_status_code == reqwest::StatusCode::NOT_FOUND {
        println!("404"); //Not Found
    }
    if error_status_code == reqwest::StatusCode::METHOD_NOT_ALLOWED {
        println!("405"); //Method Not Allowed
    }
    if error_status_code == reqwest::StatusCode::NOT_ACCEPTABLE {
        println!("406"); //Not Acceptable
    }
    if error_status_code == reqwest::StatusCode::PROXY_AUTHENTICATION_REQUIRED {
        println!("407"); //Proxy Authentication Required
    }
    if error_status_code == reqwest::StatusCode::REQUEST_TIMEOUT {
        println!("408"); //Request Timeout
    }
    if error_status_code == reqwest::StatusCode::CONFLICT {
        println!("409"); //Conflict
    }
    if error_status_code == reqwest::StatusCode::GONE {
        println!("410"); //Gone
    }
    if error_status_code == reqwest::StatusCode::LENGTH_REQUIRED {
        println!("411"); //Length Required
    }
    if error_status_code == reqwest::StatusCode::PRECONDITION_FAILED {
        println!("412"); //Precondition Failed
    }
    if error_status_code == reqwest::StatusCode::PAYLOAD_TOO_LARGE {
        println!("413"); //Payload Too Large
    }
    if error_status_code == reqwest::StatusCode::URI_TOO_LONG {
        println!("414"); //URI Too Long
    }
    if error_status_code == reqwest::StatusCode::UNSUPPORTED_MEDIA_TYPE {
        println!("415"); //Unsupported Media Type
    }
    if error_status_code == reqwest::StatusCode::RANGE_NOT_SATISFIABLE {
        println!("416"); //Range Not Satisfiable
    }
    if error_status_code == reqwest::StatusCode::EXPECTATION_FAILED {
        println!("417"); //Expectation Failed
    }
    if error_status_code == reqwest::StatusCode::IM_A_TEAPOT {
        //что это за херня???????
        println!("418"); //I’m a teapot
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("419"); //Authentication Timeout
    // }
    //да, нет 420 хз поч
    if error_status_code == reqwest::StatusCode::MISDIRECTED_REQUEST {
        println!("421"); //Misdirected Request
    }
    if error_status_code == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        println!("422"); //Unprocessable Entity
    }
    if error_status_code == reqwest::StatusCode::LOCKED {
        println!("423"); //Locked
    }
    if error_status_code == reqwest::StatusCode::FAILED_DEPENDENCY {
        println!("424"); //Failed Dependency
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("425"); //Too Early
    // }
    if error_status_code == reqwest::StatusCode::UPGRADE_REQUIRED {
        println!("426"); //Upgrade Required
    }
    //да, нет 427 хз поч
    if error_status_code == reqwest::StatusCode::PRECONDITION_REQUIRED {
        println!("428"); //Precondition Required
    }
    if error_status_code == reqwest::StatusCode::TOO_MANY_REQUESTS {
        println!("429"); //Too Many Requests
    }
    //да, нет 430 хз поч
    if error_status_code == reqwest::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE {
        println!("431"); //Request Header Fields Too Large
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("449"); //Retry With
    // }
    if error_status_code == reqwest::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS {
        println!("451"); //Unavailable For Legal Reasons
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("499"); //Client Closed Request
    // }
    // 5xx: Server Error (ошибка сервера):
    if error_status_code == reqwest::StatusCode::INTERNAL_SERVER_ERROR {
        println!("500"); //Internal Server Error
    }
    if error_status_code == reqwest::StatusCode::NOT_IMPLEMENTED {
        println!("501"); //Not Implemented
    }
    if error_status_code == reqwest::StatusCode::BAD_GATEWAY {
        println!("502"); //Bad Gateway
    }
    if error_status_code == reqwest::StatusCode::SERVICE_UNAVAILABLE {
        println!("503"); //Service Unavailable
    }
    if error_status_code == reqwest::StatusCode::GATEWAY_TIMEOUT {
        println!("504"); //Gateway Timeout
    }
    if error_status_code == reqwest::StatusCode::HTTP_VERSION_NOT_SUPPORTED {
        println!("505"); //HTTP Version Not Supported
    }
    if error_status_code == reqwest::StatusCode::VARIANT_ALSO_NEGOTIATES {
        println!("506"); //Variant Also Negotiates
    }
    if error_status_code == reqwest::StatusCode::INSUFFICIENT_STORAGE {
        println!("507"); //Insufficient Storage
    }
    if error_status_code == reqwest::StatusCode::LOOP_DETECTED {
        println!("508"); //Loop Detected
    }
    // if error_status_code == reqwest::StatusCode:: {//почему то в реквесте этого нет
    //     println!("509"); //Bandwidth Limit Exceeded
    // }
    if error_status_code == reqwest::StatusCode::NOT_EXTENDED {
        println!("510"); //Not Extended
    }
    if error_status_code == reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED {
        println!("511"); //Network Authentication Required
    }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("520"); //Unknown Error
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("521"); //Web Server Is Down
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("522"); //Connection Timed Out
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("523"); //Origin Is Unreachable
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("524"); //A Timeout Occurred
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("525"); //SSL Handshake Failed
    // }
    // if error_status_code == reqwest::StatusCode:: {
    //     println!("526"); //Invalid SSL Certificate
    // }
    false
}
