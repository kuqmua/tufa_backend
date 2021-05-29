use html_parser::{Dom, Node};

pub fn parse_github_html(option_content: Option<String>) {
    // let mut avatar_link: Option<String> = None;
    match option_content {
        Some(content) => {
            let result_content = Dom::parse(&content);
            match result_content {
                Ok(dom) => {
                    match dom.children.first() {
                        Some(element1) => {
                            if let Node::Element(ref element2) = element1 {
                                match element2.children.first() {
                                    Some(element3) => {
                                        if let Node::Element(ref element4) = element3 {
                                            match element4.children.first() {
                                                Some(element5) => {
                                                    if let Node::Element(ref element6) = element5 {
                                                        match element6.children.len() {
                                                            2 => {
                                                                parse_github_html_first_part(
                                                                    &element6.children[0],
                                                                );
                                                                parse_github_html_second_part(
                                                                    &element6.children[1],
                                                                );
                                                            }
                                                            _ => {
                                                                println!("_____should handle what or ...?");
                                                            }
                                                        }
                                                    }
                                                }
                                                None => {
                                                    println!("fn")
                                                }
                                            }
                                        }
                                    }
                                    None => {
                                        println!("fn")
                                    }
                                }
                            }
                        }
                        None => {
                            println!("fn")
                        }
                    }
                }
                Err(e) => {
                    println!("Dom::parse error {}", e)
                }
            }
        }
        None => {
            println!("fn")
        }
    }
    // println!("avatar_link {:#?}",avatar_link);
}

pub fn parse_github_html_first_part(first_child: &Node) {
    match first_child {
        Node::Element(ref element7twochildrenfirst) => {
            match element7twochildrenfirst.children.first() {
                Some(element8twochildrenfirst) => {
                    if let Node::Element(ref element9first) = element8twochildrenfirst {
                        match element9first.children.first() {
                            Some(element10first) => {
                                if let Node::Element(ref _element11first) = element10first {
                                    // avatar_link = element11first.attributes["src"].clone();
                                }
                            }
                            None => {
                                println!("fn")
                            }
                        }
                    }
                }
                None => {
                    println!("fn")
                }
            }
        }
        _ => {
            println!("___")
        }
    }
}

pub fn parse_github_html_second_part(second_child: &Node) {
    let mut author: &str = "noone";
    let mut action: &str = "noaction";
    let mut repository: &str = "norepository";
    let mut datejs: Option<String> = None;
    let mut date: &str = "nodate";
    let mut actionto: &str = "noactionto";
    let mut branch: &str = "nobranch";
    match second_child {
        Node::Element(ref second_child_element1) => {
            match second_child_element1.children.len() {
                1 => {
                    // println!("1");
                    match second_child_element1.children[0] {
                        Node::Element(ref second_child_element2) => {
                            match second_child_element2.children.len() {
                                5 => {
                                    // println!("5");
                                    match second_child_element2.children[0] {
                                        Node::Element(ref second_child_element3first) => {
                                            match second_child_element3first.children.len() {
                                                1 => match second_child_element3first.children[0] {
                                                    Node::Text(ref texttext) => {
                                                        author = texttext;
                                                    }
                                                    _ => println!("diff node eee2"),
                                                },
                                                _ => println!(
                                                    "diff3 {}",
                                                    second_child_element3first.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[1] {
                                        Node::Text(ref second_child_element3second) => {
                                            action = second_child_element3second
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[2] {
                                        Node::Element(ref second_child_element3third) => {
                                            match second_child_element3third.children.len() {
                                                1 => match second_child_element3third.children[0] {
                                                    Node::Text(ref texttext) => {
                                                        repository = texttext;
                                                    }
                                                    _ => println!("something else2"),
                                                },
                                                _ => println!(
                                                    "diff3 {}",
                                                    second_child_element3third.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[3] {
                                        Node::Element(ref second_child_element3fourth) => {
                                            match second_child_element3fourth.children.len() {
                                                1 => {
                                                    match second_child_element3fourth.children[0] {
                                                        Node::Element(
                                                            ref second_child_element4fourth,
                                                        ) => {
                                                            // println!(
                                                            //     "second_child_element4fourth {:#?}",
                                                            //     second_child_element4fourth
                                                            // );
                                                            datejs = second_child_element4fourth
                                                                .attributes["datetime"]
                                                                .clone();
                                                            match second_child_element4fourth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4fourth
                                                                .children[0] {
                                                                    Node::Text(ref second_child_element5fourth) => {
                                                                        date = second_child_element5fourth;
                                                                    }
                                                                    _ => println!("diff node"),
                                                                }
                                                                }
                                                                _ => println!(
                                                                    "diff4 {}",
                                                                    second_child_element4fourth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("something else2"),
                                                    }
                                                }
                                                _ => println!(
                                                    "diff3 {}",
                                                    second_child_element3fourth.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[4] {
                                        Node::Element(ref second_child_element3firth) => {
                                            match second_child_element3firth.children.len() {
                                                3 => {
                                                    match second_child_element3firth.children[0] {
                                                        Node::Element(ref second_child_element4firth) => {
                                                            match second_child_element4firth.children.len() {
                                                                1 => {
                                                                    match second_child_element4firth.children[0]{
                                                                        Node::Text(ref texttext) => {
                                                                            actionto = texttext;
                                                                        }
                                                                        _ => println!("diff node eee2"),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff4 {}",
                                                                    second_child_element4firth.children.len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node"),
                                                        
                                                    }
                                                    // println!("second_child_element3firth.children[1] {:#?}", second_child_element3firth.children[1]);
                                                    match second_child_element3firth.children[1] {
                                                        Node::Element(ref second_child_element4firth) => {
                                                            match second_child_element4firth.children.len() {
                                                                1 => {
                                                                    match second_child_element4firth.children[0]{
                                                                        Node::Text(ref texttext) => {
                                                                            branch = texttext;
                                                                        }
                                                                        _ => println!("diff node eee2"),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff4 {}",
                                                                    second_child_element4firth.children.len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node"),
                                                    }
                                                    // match second_child_element3firth.children[2] {

                                                    // }
                                                }
                                                _ => println!(
                                                    "diff4 {}",
                                                    second_child_element3firth.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    // match second_child_element2.children[4] {}
                                }
                                _ => {
                                    //something here exists
                                    println!("diff2 {}", second_child_element2.children.len());
                                }
                            }
                        }
                        _ => println!("something else1"),
                    }
                }
                2 => {
                    println!("2f");
                }
                5 => {
                    println!("5f");
                }
                _ => {
                    println!("diff {}", second_child_element1.children.len());
                }
            }
        }
        _ => {
            println!("___")
        }
    }
    // println!("author {}", author);
    // println!("action {}", action);
    // println!("repository {}", repository);
    // println!("datejs {:#?}", datejs);
    // println!("date {}", date);
    // println!("repository {}", repository);
    // println!("actionto {}", actionto);
    println!("branch {}", branch);
}
