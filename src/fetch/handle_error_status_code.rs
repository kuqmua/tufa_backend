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

pub fn handle_error_status_code(error_status_code: u16) -> bool {
    // println!(" handle_error_status_code {}", error_status_code)
    if error_status_code == 100 {
        println!("100"); //Continue
    }
    if error_status_code == 101 {
        println!("101"); //Switching Protocols
    }
    if error_status_code == 102 {
        println!("102"); //Processing
    }
    if error_status_code == 103 {
        println!("103"); //Early Hints («ранняя метаинформация»);
    }
    if error_status_code == 200 {
        println!("200"); // Success
    }
    if error_status_code == 201 {
        println!("201"); //Created
    }
    if error_status_code == 202 {
        println!("202"); //Accepted
    }
    if error_status_code == 203 {
        println!("203"); //Non-Authoritative Information
    }
    if error_status_code == 204 {
        println!("204"); //No Content
    }
    if error_status_code == 205 {
        println!("205"); //Reset Content
    }
    if error_status_code == 206 {
        println!("206"); //Partial Content
    }
    if error_status_code == 207 {
        println!("207"); //Multi-Status
    }
    if error_status_code == 208 {
        println!("208"); //Already Reported
    }
    if error_status_code == 226 {
        println!("226"); //IM Used
    }
    //Redirection
    if error_status_code == 300 {
        println!("300"); //Multiple Choices
    }
    if error_status_code == 301 {
        println!("301"); //Moved Permanently
    }
    if error_status_code == 302 {
        println!("302"); //Moved Temporarily
    }
    // if error_status_code == 302 {//2 302 кода чего ????
    //     println!("302");//Found
    // }
    if error_status_code == 303 {
        println!("303"); //See Other
    }
    if error_status_code == 304 {
        println!("304"); //Not Modified
    }
    if error_status_code == 305 {
        println!("305"); //Use Proxy
    }
    if error_status_code == 306 {
        println!("306"); //— зарезервировано (код использовался только в ранних спецификациях)
    }
    if error_status_code == 307 {
        println!("307"); //Temporary Redirect
    }
    if error_status_code == 308 {
        println!("308"); //Permanent Redirect
    }
    // 4xx: Client Error (ошибка клиента):
    if error_status_code == 400 {
        println!("400"); //Bad Request
    }
    if error_status_code == 401 {
        println!("401"); //Unauthorized
    }
    if error_status_code == 402 {
        println!("402"); //Payment Required
    }
    if error_status_code == 403 {
        println!("403"); //Forbidden
    }
    if error_status_code == 404 {
        println!("404"); //Not Found
    }
    if error_status_code == 405 {
        println!("405"); //Method Not Allowed
    }
    if error_status_code == 406 {
        println!("406"); //Not Acceptable
    }
    if error_status_code == 407 {
        println!("407"); //Proxy Authentication Required
    }
    if error_status_code == 408 {
        println!("408"); //Request Timeout
    }
    if error_status_code == 409 {
        println!("409"); //Conflict
    }
    if error_status_code == 410 {
        println!("410"); //Gone
    }
    if error_status_code == 411 {
        println!("411"); //Length Required
    }
    if error_status_code == 412 {
        println!("412"); //Precondition Failed
    }
    if error_status_code == 413 {
        println!("413"); //Payload Too Large
    }
    if error_status_code == 414 {
        println!("414"); //URI Too Long
    }
    if error_status_code == 415 {
        println!("415"); //Unsupported Media Type
    }
    if error_status_code == 416 {
        println!("416"); //Range Not Satisfiable
    }
    if error_status_code == 417 {
        println!("417"); //Expectation Failed
    }
    if error_status_code == 418 {
        println!("418"); //I’m a teapot
    }
    if error_status_code == 419 {
        println!("419"); //Authentication Timeout
    }
    //да, нет 420 хз поч
    if error_status_code == 421 {
        println!("421"); //Misdirected Request
    }
    if error_status_code == 422 {
        println!("422"); //Unprocessable Entity
    }
    if error_status_code == 423 {
        println!("423"); //Locked
    }
    if error_status_code == 424 {
        println!("424"); //Failed Dependency
    }
    if error_status_code == 425 {
        println!("425"); //Too Early
    }
    if error_status_code == 426 {
        println!("426"); //Upgrade Required
    }
    //да, нет 427 хз поч
    if error_status_code == 428 {
        println!("428"); //Precondition Required
    }
    if error_status_code == 429 {
        println!("429"); //Too Many Requests
    }
    //да, нет 430 хз поч
    if error_status_code == 431 {
        println!("431"); //Request Header Fields Too Large
    }
    if error_status_code == 449 {
        println!("449"); //Retry With
    }
    if error_status_code == 451 {
        println!("451"); //Unavailable For Legal Reasons
    }
    if error_status_code == 499 {
        println!("499"); //Client Closed Request
    }
    // 5xx: Server Error (ошибка сервера):

    if error_status_code == 500 {
        println!("500"); //Internal Server Error
    }
    if error_status_code == 501 {
        println!("501"); //Not Implemented
    }
    if error_status_code == 502 {
        println!("502"); //Bad Gateway
    }
    if error_status_code == 503 {
        println!("503"); //Service Unavailable
    }
    if error_status_code == 504 {
        println!("504"); //Gateway Timeout
    }
    if error_status_code == 505 {
        println!("505"); //HTTP Version Not Supported
    }
    if error_status_code == 506 {
        println!("506"); //Variant Also Negotiates
    }
    if error_status_code == 507 {
        println!("507"); //Insufficient Storage
    }
    if error_status_code == 508 {
        println!("508"); //Loop Detected
    }
    if error_status_code == 509 {
        println!("509"); //Bandwidth Limit Exceeded
    }
    if error_status_code == 510 {
        println!("510"); //Not Extended
    }
    if error_status_code == 511 {
        println!("511"); //Network Authentication Required
    }
    if error_status_code == 520 {
        println!("520"); //Unknown Error
    }
    if error_status_code == 521 {
        println!("521"); //Web Server Is Down
    }
    if error_status_code == 522 {
        println!("522"); //Connection Timed Out
    }
    if error_status_code == 523 {
        println!("523"); //Origin Is Unreachable
    }
    if error_status_code == 524 {
        println!("524"); //A Timeout Occurred
    }
    if error_status_code == 525 {
        println!("525"); //SSL Handshake Failed
    }
    if error_status_code == 526 {
        println!("526"); //Invalid SSL Certificate
    }
    false
}
