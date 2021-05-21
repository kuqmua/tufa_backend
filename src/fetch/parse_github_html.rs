use html_parser::{Dom, Node};

pub fn parse_github_html(option_content: Option<String>) {
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
                                                            0 => {
                                                                println!("zero");
                                                            }
                                                            2 => {
                                                                match element6.children[0] {
                                                                    Node::Element(
                                                                        ref element7first,
                                                                    ) => {
                                                                        match element7first
                                                                            .children
                                                                            .first()
                                                                        {
                                                                            Some(element8first) => {
                                                                                if let Node::Element(ref  element9first) =  element8first {
                                                                                      println!(
                                                                            "element9first {:#?}",
                                                                            element9first
                                                                        );
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
                                                                match element6.children[1] {
                                                                    Node::Element(
                                                                        ref element7second,
                                                                    ) => {
                                                                        // println!(
                                                                        //     "element7second {:#?}",
                                                                        //     element7second
                                                                        // )
                                                                    }
                                                                    _ => {
                                                                        println!("___")
                                                                    }
                                                                }
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
