use html_parser::{Dom, Node};
// use select::document::Document;
// use select::predicate::{Attr, Class, Name, Predicate};
// use serde::__private::ser::constrain;

// pub fn parse_github_html(option_content: Option<String>) {
//     match option_content {
//         Some(content) => {
//             // println!("content{}", content);
//             //include_str!("s.html")
//             let bbb: &str = &content;
//             let document = Document::from(bbb);
//             for node in document.find(Class("mr12")) {
//                 //.descendant(Name("a"))
//                 // println!("{} ({:?})", node.text(), node.attr("href").unwrap());
//                 // println!("{:#?}", node)
//             }
//             // println!();
//             // println!("# Top 5 Questions");
//             // for node in document.find(Class("question-summary")).take(5) {
//             //     let question = node.find(Class("question-hyperlink")).next().unwrap();
//             //     let votes = node.find(Class("vote-count-post")).next().unwrap().text();
//             //     let answers = node
//             //         .find(Class("status").descendant(Name("strong")))
//             //         .next()
//             //         .unwrap()
//             //         .text();
//             //     let tags = node
//             //         .find(Class("post-tag"))
//             //         .map(|tag| tag.text())
//             //         .collect::<Vec<_>>();
//             //     let asked_on = node.find(Class("relativetime")).next().unwrap().text();
//             //     let asker = node
//             //         .find(Class("user-details").descendant(Name("a")))
//             //         .next()
//             //         .unwrap()
//             //         .text();
//             //     println!(" Question: {}", question.text());
//             //     println!("  Answers: {}", answers);
//             //     println!("    Votes: {}", votes);
//             //     println!("   Tagged: {}", tags.join(", "));
//             //     println!(" Asked on: {}", asked_on);
//             //     println!("    Asker: {}", asker);
//             //     println!(
//             //         "Permalink: http://stackoverflow.com{}",
//             //         question.attr("href").unwrap()
//             //     );
//             //     println!();
//             // }
//             // println!("# Top 10 Related Tags");
//             // for node in document
//             //     .find(Attr("id", "h-related-tags"))
//             //     .next()
//             //     .unwrap()
//             //     .parent()
//             //     .unwrap()
//             //     .find(Name("div"))
//             //     .take(10)
//             // {
//             //     let tag = node.find(Name("a")).next().unwrap().text();
//             //     let count = node
//             //         .find(Class("item-multiplier-count"))
//             //         .next()
//             //         .unwrap()
//             //         .text();
//             //     println!("{} ({})", tag, count);
//             // }
//         }
//         None => {
//             println!("pressf")
//         }
//     }
// }

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
    let mut avatar_link: Option<String> = None;
    match first_child {
        Node::Element(ref element7twochildrenfirst) => {
            match element7twochildrenfirst.children.first() {
                Some(element8twochildrenfirst) => {
                    if let Node::Element(ref element9first) = element8twochildrenfirst {
                        match element9first.children.first() {
                            Some(element10first) => {
                                if let Node::Element(ref element11first) = element10first {
                                    match element11first.attributes.get("src") {
                                        Some(value) => {
                                            avatar_link = value.clone();
                                        }
                                        None => println!("todo4"),
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
    let mut release_tag: &str = "noreleasetag";
    let mut of: &str = "of";
    let mut bot_tag: &str = "nobotTag";
    match second_child {
        Node::Element(ref second_child_element1) => {
            match second_child_element1.children.len() {
                1 => {
                    match second_child_element1.children[0] {
                        Node::Element(ref second_child_element2) => {
                            match second_child_element2.children.len() {
                                5 => {
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
                                                            match second_child_element4fourth
                                                                .attributes
                                                                .get("datetime")
                                                            {
                                                                Some(value) => {
                                                                    datejs = value.clone();
                                                                }
                                                                None => println!("todo5"),
                                                            }

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
                                                                    "diff48 {}",
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
                                                        Node::Element(
                                                            ref second_child_element4firth,
                                                        ) => {
                                                            match second_child_element4firth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4firth
                                                                        .children[0]
                                                                    {
                                                                        Node::Text(
                                                                            ref texttext,
                                                                        ) => {
                                                                            actionto = texttext;
                                                                        }
                                                                        _ => println!(
                                                                            "diff node eee2"
                                                                        ),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff47 {}",
                                                                    second_child_element4firth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node"),
                                                    }
                                                    match second_child_element3firth.children[1] {
                                                        Node::Element(
                                                            ref second_child_element4firth,
                                                        ) => {
                                                            match second_child_element4firth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4firth
                                                                        .children[0]
                                                                    {
                                                                        Node::Text(
                                                                            ref texttext,
                                                                        ) => {
                                                                            branch = texttext;
                                                                        }
                                                                        _ => println!(
                                                                            "diff node eee2"
                                                                        ),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff46 {}",
                                                                    second_child_element4firth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node"),
                                                    }
                                                    match second_child_element3firth.children[2] {
                                                        Node::Element(
                                                            ref second_child_element4firth,
                                                        ) => {
                                                            match second_child_element4firth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4firth
                                                                        .children[0]
                                                                    {
                                                                        Node::Element(
                                                                            ref
                                                                            second_child_element5firth,
                                                                        ) => {
                                                                            for i in &second_child_element5firth.children {
                                                                                parse_github_html_second_part_inner_one_element(&i);
                                                                            }
                                                                        }
                                                                        _ => println!("diff node"),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff45 {}",
                                                                    second_child_element4firth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node"),
                                                    }
                                                }
                                                _ => println!(
                                                    "diff49 {}",
                                                    second_child_element3firth.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                }
                                6 => {
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
                                        Node::Element(ref second_child_element3first) => {
                                            match second_child_element3first.children.len() {
                                                1 => match second_child_element3first.children[0] {
                                                    Node::Text(ref texttext) => {
                                                        bot_tag = texttext;
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
                                    match second_child_element2.children[2] {
                                        Node::Text(ref second_child_element3second) => {
                                            action = second_child_element3second
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[3] {
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
                                    match second_child_element2.children[4] {
                                        Node::Element(ref second_child_element3fourth) => {
                                            match second_child_element3fourth.children.len() {
                                                1 => {
                                                    match second_child_element3fourth.children[0] {
                                                        Node::Element(
                                                            ref second_child_element4fourth,
                                                        ) => {
                                                            match second_child_element4fourth
                                                                .attributes
                                                                .get("datetime")
                                                            {
                                                                Some(value) => {
                                                                    datejs = value.clone();
                                                                }
                                                                None => println!("todo5"),
                                                            }

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
                                                                    "diff48 {}",
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
                                    match second_child_element2.children[5] {
                                        Node::Element(ref second_child_element3firth) => {
                                            match second_child_element3firth.children.len() {
                                                3 => {
                                                    match second_child_element3firth.children[0] {
                                                        Node::Element(
                                                            ref second_child_element4firth,
                                                        ) => {
                                                            match second_child_element4firth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4firth
                                                                        .children[0]
                                                                    {
                                                                        Node::Text(
                                                                            ref texttext,
                                                                        ) => {
                                                                            actionto = texttext;
                                                                        }
                                                                        _ => println!(
                                                                            "diff node eee2"
                                                                        ),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff47 {}",
                                                                    second_child_element4firth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node qqq"),
                                                    }
                                                    match second_child_element3firth.children[1] {
                                                        Node::Element(
                                                            ref second_child_element4firth,
                                                        ) => {
                                                            match second_child_element4firth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4firth
                                                                        .children[0]
                                                                    {
                                                                        Node::Text(
                                                                            ref texttext,
                                                                        ) => {
                                                                            branch = texttext;
                                                                        }
                                                                        _ => println!(
                                                                            "diff node eee2"
                                                                        ),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff46 {}",
                                                                    second_child_element4firth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node www"),
                                                    }
                                                    match second_child_element3firth.children[2] {
                                                        Node::Element(
                                                            ref second_child_element4firth,
                                                        ) => {
                                                            match second_child_element4firth
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match second_child_element4firth
                                                                        .children[0]
                                                                    {
                                                                        Node::Element(
                                                                            ref
                                                                            second_child_element5firth,
                                                                        ) => {
                                                                            for i in &second_child_element5firth.children {
                                                                                parse_github_html_second_part_inner_one_element(&i);
                                                                            }
                                                                        }
                                                                        _ => println!("diff node tttt"),
                                                                    }
                                                                }
                                                                _ => println!(
                                                                    "diff45 {}",
                                                                    second_child_element4firth
                                                                        .children
                                                                        .len()
                                                                ),
                                                            }
                                                        }
                                                        _ => println!("diff node eee"),
                                                    }
                                                }
                                                _ => println!(
                                                    "diff49 {}",
                                                    second_child_element3firth.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                }
                                _ => {
                                    //something here exists
                                    println!("diff25.. {}", second_child_element2.children.len());
                                }
                            }
                        }
                        _ => println!("something else1"),
                    }
                }
                2 => {
                    match second_child_element1.children[0] {
                        Node::Element(ref second_child_element2) => {
                            match second_child_element2.children.len() {
                                1 => match second_child_element2.children[0] {
                                    Node::Element(ref second_child_element3) => {
                                        match second_child_element3.children.len() {
                                            1 => {
                                                two_elements_one_child(
                                                    &second_child_element3.children[0],
                                                );
                                            }
                                            4 => {
                                                two_elements_four_children_first(
                                                    &second_child_element3.children[0],
                                                );
                                                two_elements_four_children_second(
                                                    &second_child_element3.children[1],
                                                );
                                                two_elements_four_children_third(
                                                    &second_child_element3.children[2],
                                                );
                                                two_elements_four_children_fourth(
                                                    &second_child_element3.children[3],
                                                );
                                            }
                                            _ => {
                                                println!(
                                                    "diff255pp {}",
                                                    second_child_element3.children.len()
                                                );
                                            }
                                        }
                                    }
                                    _ => println!("diff node"),
                                },
                                6 => {
                                    // println!("six{:#?}", second_child_element2.children);
                                    match second_child_element2.children[0] {
                                        Node::Element(ref second_child_element3) => {
                                            match second_child_element3.children.len() {
                                                1 => match second_child_element3.children[0] {
                                                    Node::Text(ref pppp) => {
                                                        // println!("pppp{}", pppp);
                                                        author = pppp;
                                                    }
                                                    _ => println!("diff node"),
                                                },
                                                _ => println!(
                                                    "diff255yy {}",
                                                    second_child_element3.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[1] {
                                        Node::Text(ref second_child_element3) => {
                                            action = second_child_element3;
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[2] {
                                        Node::Element(ref second_child_element3) => {
                                            match second_child_element3.children.len() {
                                                1 => match second_child_element3.children[0] {
                                                    Node::Text(ref pppp) => {
                                                        // println!("pppp{}", pppp);
                                                        release_tag = pppp;
                                                    }
                                                    _ => println!("diff node"),
                                                },
                                                _ => println!(
                                                    "diff255yy {}",
                                                    second_child_element3.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[3] {
                                        Node::Text(ref second_child_element3) => {
                                            of = second_child_element3;
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[4] {
                                        Node::Element(ref second_child_element3) => {
                                            match second_child_element3.children.len() {
                                                1 => match second_child_element3.children[0] {
                                                    Node::Text(ref pppp) => {
                                                        // println!("pppp{}", pppp);
                                                        repository = pppp;
                                                    }
                                                    _ => println!("diff node"),
                                                },
                                                _ => println!(
                                                    "diff255yy {}",
                                                    second_child_element3.children.len()
                                                ),
                                            }
                                        }
                                        _ => println!("diff node"),
                                    }
                                    match second_child_element2.children[5] {
                                        Node::Element(ref second_child_element3fourth) => {
                                            match second_child_element3fourth.children.len() {
                                                1 => {
                                                    match second_child_element3fourth.children[0] {
                                                        Node::Element(
                                                            ref second_child_element4fourth,
                                                        ) => {
                                                            match second_child_element4fourth
                                                                .attributes
                                                                .get("datetime")
                                                            {
                                                                Some(value) => {
                                                                    datejs = value.clone();
                                                                }
                                                                None => println!("todo6"),
                                                            }
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
                                                                    "diff48 {}",
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
                                }
                                _ => println!("diff255yy {}", second_child_element2.children.len()),
                            }
                        }
                        _ => println!("diff node"),
                    }
                    //here todo 2
                    // println!("2f");
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
    // println!("branch {}", branch);
}

pub fn parse_github_html_second_part_inner_one_element(inner_one_element: &Node) {
    //out params need to be pushed into vec array or something
    let mut avatar_link: Option<String> = None;
    let mut relative_commit_link: Option<String> = None;
    let mut commit_text: &str = "nocommittext";
    let mut from_text: &str = "nofromtext";
    let mut commits_number: &str = "nonumberofcommits";
    let mut isssue_label: &str = "isssuelabel"; //todo or isssuelabel in struct by default in html
    match inner_one_element {
        Node::Element(ref inner_one_element1) => {
            match inner_one_element1.children.len() {
                1 => {
                    // println!(
                    //     "inner_one_element1.children[0] {:#?}",
                    //     inner_one_element1.children[0]
                    // )
                    match inner_one_element1.children[0] {
                        Node::Element(ref inner_one_element1_one_element_first) => {
                            match inner_one_element1_one_element_first.children.len() {
                                1 => match inner_one_element1_one_element_first.children[0] {
                                    Node::Text(ref pppp) => {
                                        commits_number = pppp;
                                    }
                                    _ => println!("diff node"),
                                },
                                _ => println!(
                                    "diff26671 {}",
                                    inner_one_element1_one_element_first.children.len()
                                ),
                            }
                        }
                        _ => println!("diff node"),
                    }
                }
                3 => {
                    // println!(
                    //     "inner_one_element1.children[0] {:#?}",
                    //     inner_one_element1.children[0]
                    // );
                    match inner_one_element1.children[0] {
                        //todo 1 and 2
                        Node::Element(ref inner_one_element1first) => {
                            match inner_one_element1first.children.len() {
                                1 => match inner_one_element1first.children[0] {
                                    Node::Element(ref inner_one_element1first1) => {
                                        match inner_one_element1first1.children.len() {
                                            0 => {}
                                            1 => match inner_one_element1first1.children[0] {
                                                Node::Element(ref inner_one_element1first11) => {
                                                    match inner_one_element1first11
                                                        .attributes
                                                        .get("src")
                                                    {
                                                        Some(value) => {
                                                            avatar_link = value.clone();
                                                        }
                                                        None => println!("todo7"),
                                                    }
                                                }
                                                _ => println!("diff node"),
                                            },
                                            _ => {
                                                println!(
                                                    "diff24 {}",
                                                    inner_one_element1first1.children.len()
                                                );
                                            }
                                        }
                                    }
                                    _ => println!("diff node"),
                                },
                                _ => {
                                    println!("diff23 {}", inner_one_element1first.children.len());
                                }
                            }
                        }
                        _ => println!("diff node uuuuuu"),
                    }
                    match inner_one_element1.children[1] {
                        Node::Element(ref inner_one_element1second) => {
                            match inner_one_element1second.children.len() {
                                1 => match inner_one_element1second.children[0] {
                                    Node::Element(ref inner_one_element1second1) => {
                                        match inner_one_element1second1.attributes.get("href") {
                                            Some(value) => {
                                                relative_commit_link = value.clone();
                                            }
                                            None => println!("todo8"),
                                        }
                                    }
                                    _ => println!("diff node"),
                                },
                                _ => {
                                    println!("diff22 {}", inner_one_element1second.children.len());
                                }
                            }
                        }
                        _ => println!("diff node"),
                    }
                    match inner_one_element1.children[2] {
                        Node::Element(ref inner_one_element1third) => {
                            match inner_one_element1third.children.len() {
                                1 => match inner_one_element1third.children[0] {
                                    Node::Element(ref inner_one_element1third1) => {
                                        match inner_one_element1third1.children.len() {
                                            1 => {
                                                commit_text = handle_text_element(
                                                    &inner_one_element1third1.children[0],
                                                );
                                            }
                                            2 => {
                                                // println!(
                                                //     "todo 2uuu {:#?}",
                                                //     inner_one_element1third1.children
                                                // );
                                                commit_text = handle_text_element(
                                                    &inner_one_element1third1.children[0],
                                                );
                                                second_element(
                                                    &inner_one_element1third1.children[1],
                                                );
                                            }
                                            3 => {
                                                commit_text = handle_text_element(
                                                    &inner_one_element1third1.children[0],
                                                );
                                                second_element(
                                                    &inner_one_element1third1.children[1],
                                                );
                                                // println!(
                                                //     "🚀inner_one_element1third1.children[2]{:#?}",
                                                //     inner_one_element1third1.children[2]
                                                // );
                                                //here
                                                match inner_one_element1third1.children[2] {
                                                    Node::Text(ref text) => {
                                                        from_text = text;
                                                    }
                                                    Node::Element(ref something) => {
                                                        if something.name != "g-emoji" {
                                                            println!("todo not g-emoji")
                                                        }
                                                    }
                                                    _ => println!("diff node"),
                                                }

                                                // from_text = handle_text_element(
                                                //     &inner_one_element1third1.children[2],
                                                // );
                                                //and here
                                            }
                                            5 => {
                                                //todo
                                                commit_text = handle_text_element(
                                                    &inner_one_element1third1.children[0],
                                                );
                                                second_element(
                                                    &inner_one_element1third1.children[1],
                                                );
                                                from_text = handle_text_element(
                                                    &inner_one_element1third1.children[2],
                                                );
                                                second_element(
                                                    &inner_one_element1third1.children[3],
                                                );
                                                handle_text_element(
                                                    //some trash
                                                    &inner_one_element1third1.children[4],
                                                );
                                            }
                                            _ => println!(
                                                "diff21 {}",
                                                inner_one_element1third1.children.len()
                                            ),
                                        }
                                    }
                                    _ => println!("diff node oooooooo"),
                                },
                                _ => println!("diff27 {}", inner_one_element1third.children.len()),
                            }
                        }
                        _ => println!("diff node iiiiiiii"),
                    }
                }
                _ => println!("diff26 {}", inner_one_element1.children.len()),
            }
        }
        _ => println!("diff node yyyyyyy"),
    }
    // println!("avatar_link {:#?}", avatar_link);
    // println!("relative_commit_link {:#?}", relative_commit_link);
    // println!("commit_text {:#?}", commit_text);
    // println!("from_text {:#?}", from_text);
    // println!("commits_number {:#?}", commits_number);
}
pub fn handle_text_element(first_element: &Node) -> &str {
    let mut text_handle: &str = "";
    match first_element {
        Node::Text(ref text) => {
            text_handle = text;
        }
        _ => println!("diff node"),
    }
    text_handle
}
pub fn second_element(second_element: &Node) {
    let mut data_hovercard_type: Option<String> = None;
    let mut data_hovercard_url: Option<String> = None;
    let mut data_id: Option<String> = None;
    let mut href: Option<String> = None;
    let mut data_url: Option<String> = None;
    match second_element {
        Node::Element(ref second_element1) => {
            if let Some(value) = second_element1.attributes.get("data-hovercard-type") {
                data_hovercard_type = value.clone();
            }

            if let Some(value) = second_element1.attributes.get("data-hovercard-url") {
                data_hovercard_url = value.clone();
            }

            if let Some(value) = second_element1.attributes.get("data-id") {
                data_id = value.clone();
            }

            if let Some(value) = second_element1.attributes.get("href") {
                href = value.clone();
            }

            if let Some(value) = second_element1.attributes.get("data-url") {
                data_url = value.clone();
            }
            match second_element1.children.len() {
                1 => match second_element1.children[0] {
                    Node::Text(_) => {}
                    _ => println!("diff node"),
                },
                _ => println!("diff children len {}", second_element1.children.len()),
            }
        }
        _ => println!("diff node"),
    }
    // println!("data_hovercard_type {:#?}", data_hovercard_type);
    // println!("data_hovercard_url {:#?}", data_hovercard_url);
    // println!("data_id {:#?}", data_id);
    // println!("href {:#?}", href);
    // println!("data_url {:#?}", data_url);
}
pub fn two_elements_one_child(element: &Node) {
    let mut author_name_another: &str = "noauthornameanother";
    let mut action_another: &str = "noactionanother";
    let mut the_accounts_repo_on_which_the_action_was_performed_relative_href: Option<String> =
        None;
    let mut the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from: Option<
        String,
    > = None;
    let mut datejs_another: Option<String> = None;
    let mut date_another: &str = "nodate";
    let mut from: &str = "nofrom";
    let mut isssue_label: &str = "isssuelabel"; //todo or isssuelabel in struct by default in html
    match element {
        Node::Element(ref element1) => {
            match element1.children.len() {
                4 => {
                    match element1.children[0] {
                        Node::Element(ref qqqqq) => match qqqqq.children.len() {
                            1 => match qqqqq.children[0] {
                                Node::Text(ref yyyyyy) => {
                                    author_name_another = yyyyyy;
                                }
                                _ => println!("diff node"),
                            },
                            _ => println!("diff26672 {}", element1.children.len()),
                        },
                        _ => println!("diff node"),
                    }
                    match element1.children[1] {
                        Node::Text(ref wwwww) => {
                            action_another = wwwww;
                        }
                        _ => println!("diff node45767"),
                    }
                    match element1.children[2] {
                        Node::Element(ref eeeeee) => match eeeeee.attributes.get("href") {
                            Some(value) => {
                                the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                    value.clone();
                            }
                            None => println!("todo14"),
                        },
                        _ => println!("diff node45767"),
                    }
                    match element1.children[3] {
                        Node::Element(ref rrrrr) => match rrrrr.children.len() {
                            1 => match rrrrr.children[0] {
                                Node::Element(ref rrrrr2) => {
                                    match rrrrr2.attributes.get("datetime") {
                                        Some(value) => {
                                            datejs_another = value.clone();
                                        }
                                        None => println!("todo15"),
                                    }
                                    match rrrrr2.children.len() {
                                        1 => match rrrrr2.children[0] {
                                            Node::Text(ref rrrrr23) => {
                                                date_another = rrrrr23;
                                            }
                                            _ => println!("diff node"),
                                        },
                                        _ => println!("diff48 {}", rrrrr2.children.len()),
                                    }
                                }
                                _ => println!("diff node45767899"),
                            },
                            _ => println!("diff26679129 {}", rrrrr.children.len()),
                        },
                        _ => println!("diff node45767"),
                    }
                }
                5 => {
                    match element1.children[0] {
                        Node::Element(ref qqqqq) => match qqqqq.children.len() {
                            1 => match qqqqq.children[0] {
                                Node::Text(ref yyyyyy) => {
                                    author_name_another = yyyyyy;
                                }
                                _ => println!("diff node"),
                            },
                            _ => println!("diff26677 {}", element1.children.len()),
                        },
                        _ => println!("diff node"),
                    }
                    match element1.children[1] {
                        Node::Text(ref wwwww) => {
                            action_another = wwwww;
                        }
                        _ => println!("diff node457673"),
                    }
                    match element1.children[2] {
                        Node::Element(ref eeeeee) => match eeeeee.attributes.get("href") {
                            Some(value) => {
                                the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                    value.clone();
                            }
                            None => println!("todo16"),
                        },
                        _ => println!("diff node457672"),
                    }
                    match element1.children[3] {
                        Node::Text(ref rrrrr_another) => {
                            // println!("rrrrr_another {}", rrrrr_another)
                            from = rrrrr_another;
                        }
                        _ => println!("diff node457671"),
                    }
                    match element1.children[4] {
                        Node::Element(ref rrrrr) => match rrrrr.children.len() {
                            1 => match rrrrr.children[0] {
                                Node::Element(ref rrrrr2) => {
                                    match rrrrr2.attributes.get("datetime") {
                                        Some(value) => {
                                            datejs_another = value.clone();
                                        }
                                        None => println!("todo17"),
                                    }
                                    match rrrrr2.children.len() {
                                        1 => match rrrrr2.children[0] {
                                            Node::Text(ref rrrrr23) => {
                                                date_another = rrrrr23;
                                            }
                                            _ => println!("diff node"),
                                        },
                                        _ => println!("diff48 {}", rrrrr2.children.len()),
                                    }
                                }
                                _ => println!("diff node45767899"),
                            },
                            _ => println!("diff26679129 {}", rrrrr.children.len()),
                        },
                        _ => println!("diff node45767"),
                    }
                }
                6 => {
                    match element1.children[0] {
                        Node::Element(ref qqqqq) => match qqqqq.children.len() {
                            1 => match qqqqq.children[0] {
                                Node::Text(ref yyyyyy) => {
                                    author_name_another = yyyyyy;
                                }
                                _ => println!("diff node"),
                            },
                            _ => println!("diff26677 {}", element1.children.len()),
                        },
                        _ => println!("diff node"),
                    }
                    match element1.children[1] {
                        Node::Text(ref wwwww) => {
                            action_another = wwwww;
                        }
                        _ => println!("diff node457673"),
                    }
                    match element1.children[2] {
                        Node::Element(ref eeeeee) => match eeeeee.attributes.get("href") {
                            Some(value) => {
                                the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                    value.clone();
                            }
                            None => match eeeeee.children.len() {
                                1 => match eeeeee.children[0] {
                                    Node::Text(ref eeeeee23) => {
                                        isssue_label = eeeeee23;
                                    }
                                    _ => println!("diff node"),
                                },
                                _ => println!("diff26670356 {}", eeeeee.children.len()),
                            },
                        },
                        _ => println!("diff node457672"),
                    }
                    match element1.children[3] {
                        Node::Text(ref rrrrr_another) => {
                            // println!("rrrrr_another {}", rrrrr_another)
                            from = rrrrr_another;
                        }
                        _ => println!("diff node457671"),
                    }
                    match element1.children[4] {
                        Node::Element(ref ttttt) => match ttttt.attributes.get("href") {
                            Some(value) => {
                                the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from = value.clone();
                            }
                            None => println!("todo18"),
                        },
                        _ => println!("diff node666777"),
                    }
                    match element1.children[5] {
                        Node::Element(ref rrrrr) => match rrrrr.children.len() {
                            1 => match rrrrr.children[0] {
                                Node::Element(ref rrrrr2) => {
                                    match rrrrr2.attributes.get("datetime") {
                                        Some(value) => {
                                            datejs_another = value.clone();
                                        }
                                        None => println!("todo19"),
                                    }

                                    match rrrrr2.children.len() {
                                        1 => match rrrrr2.children[0] {
                                            Node::Text(ref rrrrr23) => {
                                                date_another = rrrrr23;
                                            }
                                            _ => println!("diff node"),
                                        },
                                        _ => println!("diff48 {}", rrrrr2.children.len()),
                                    }
                                }
                                _ => println!("diff node45767899"),
                            },
                            _ => println!("diff266799 {}", rrrrr.children.len()),
                        },
                        _ => println!("diff node45767"),
                    }
                }
                _ => println!("diff26676 {}", element1.children.len()),
            }
        }
        _ => println!("diff node"),
    }
    // println!("-----------------");
    // println!("author_name_another {:#?}", author_name_another);
    // println!("action_another {:#?}", action_another);
    // println!(
    //     "the_accounts_repo_on_which_the_action_was_performed_relative_href {:#?}",
    //     the_accounts_repo_on_which_the_action_was_performed_relative_href
    // );
    // println!(
    //     "the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from {:#?}",
    //     the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from
    // );
    // println!("datejs_another {:#?}", datejs_another);
    // println!("date_another {}", date_another);
    // println!("from {}", from);
    // println!("isssue_label {}", isssue_label);
}

pub fn two_elements_four_children_first(element: &Node) {
    let mut author_another_another: &str = "noauthoranotheranother";
    match element {
        Node::Element(ref element1) => match element1.children.len() {
            1 => match element1.children[0] {
                Node::Text(ref uuuuu) => {
                    author_another_another = uuuuu;
                }
                _ => println!("diff node"),
            },
            _ => println!("diff26675 {}", element1.children.len()),
        },
        _ => println!("diff node"),
    }
    // println!("author_another_another {}", author_another_another);
}

pub fn two_elements_four_children_second(element: &Node) {
    let mut action_another_another: &str = "noactionanotheranother";
    match element {
        Node::Text(ref uuuuu) => {
            // println!("{}", uuuuu);s
            action_another_another = uuuuu;
        }
        _ => println!("diff node"),
    }
    // println!("action_another_another {}", action_another_another);
}

pub fn two_elements_four_children_third(element: &Node) {
    let mut who_follow: &str = "nowhofollow";
    match element {
        Node::Element(ref element1) => match element1.children.len() {
            1 => match element1.children[0] {
                Node::Text(ref uuuuu) => {
                    // println!("uuuuu{}", uuuuu);
                    who_follow = uuuuu;
                }
                _ => println!("diff nodennn"),
            },
            _ => println!("diff26674 {}", element1.children.len()),
        },
        _ => println!("diff nodemm"),
    }
    // println!("who_follow {}", who_follow);
}

pub fn two_elements_four_children_fourth(element: &Node) {
    let mut who_follow: &str = "nowhofollow";
    let mut datejs_another: Option<String> = None;
    let mut date_another: &str = "nodate";
    match element {
        Node::Element(ref element1) => match element1.children.len() {
            1 => match element1.children[0] {
                Node::Element(ref rrrrr2) => {
                    match rrrrr2.attributes.get("datetime") {
                        Some(value) => {
                            datejs_another = value.clone();
                        }
                        None => println!("todo3"),
                    }
                    match rrrrr2.children.len() {
                        1 => match rrrrr2.children[0] {
                            Node::Text(ref rrrrr23) => {
                                date_another = rrrrr23;
                            }
                            _ => println!("diff node"),
                        },
                        _ => println!("diff48 {}", rrrrr2.children.len()),
                    }
                }
                _ => println!("diff 4444"),
            },
            _ => println!("diff26673 {}", element1.children.len()),
        },
        _ => println!("diff 4444"),
    }
    // println!("who_follow {}", who_follow);
    // println!("datejs_another {:#?}", datejs_another);
    // println!("date_another {}", date_another);
}
