// extern crate reqwest;
// extern crate serde;
// extern crate serde_xml_rs;

// use super::arxiv_fetch_and_parse_xml::arxiv_fetch_and_parse_xml;
// use crate::check_net::check_link::check_link;
// use crate::config::ARXIV_URL;
// use crate::config::ENABLE_PRINTS_ARXIV;

// pub fn part() -> bool {
//     if check_link(ARXIV_URL).0 {
//         if ENABLE_PRINTS_ARXIV {
//             println!("i can reach {}", ARXIV_URL);
//         };
//         let fff = arxiv_fetch_and_parse_xml();
//         if ENABLE_PRINTS_ARXIV {
//             println!("{:#?} elements in Arxiv HashMap", fff.len());
//         };
//         true //чекнуть действительно ли в векторе есть хоть шот полезное
//     } else {
//         if ENABLE_PRINTS_ARXIV {
//             println!("i cannot reach {}", ARXIV_URL);
//         };
//         false
//     }
// }
