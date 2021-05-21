use html_parser::{Dom, Node};

pub fn parse_github_html(option_content: Option<String>) {
    match option_content {
        Some(content) => {
            let result_content = Dom::parse(&content);
            match result_content {
                Ok(dom) => match dom.children.first() {
                    Some(element1) => {
                        if let Node::Element(ref element2) = element1 {
                            match element2.children.first() {
                                Some(element3) => {
                                    if let Node::Element(ref element4) = element3 {
                                        match element4.children.first() {
                                            Some(element5) => {
                                                if let Node::Element(ref element6) = element5 {
                                                    match element6.children.first() {
                                                        Some(element7) => {
                                                            println!("fffff1 {:#?}", element7);
                                                        }
                                                        None => {
                                                            println!("fn")
                                                        } //second here need to parse
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
                },
                Err(e) => {
                    println!("fe")
                }
            }
        }
        None => {
            println!("fn")
        }
    }
}
//Node::Element(ref el) => match el.children.len() {
