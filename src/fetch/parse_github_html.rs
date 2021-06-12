use crate::overriding::prints::print_error_red;
use crate::overriding::prints::print_warning_orange;
use html_parser::{Dom, Node};
// use select::document::Document;
// use select::predicate::{Attr, Class, Name, Predicate};
// use serde::__private::ser::constrain;

// pub fn parse_github_html(option_content: Option<String>) {
//     let mut avatar_link: Option<String> = None; //can be an array
//     let mut updated_at: Option<String> = None;
//     let mut date: Option<String> = None;
//     let mut author: Option<String> = None;
//     let mut author_relative_link: Option<String> = None;
//     let mut repository_with_author: Option<String> = None;
//     let mut repository_with_author_relative_link: Option<String> = None;
//     let mut branch: Option<String> = None;
//     let mut branch_relative_link: Option<String> = None;
//     let mut commit_relative_link: Option<String> = None;

//     let mut action: &str = "noaction";
//     let mut actionto: &str = "noactionto";
//     let mut release_tag: &str = "noreleasetag";
//     let mut of: &str = "of";
//     let mut bot_tag: &str = "nobotTag";

//     let mut commit_text: &str = "nocommittext";
//     let mut from_text: &str = "nofromtext";
//     let mut commits_number: &str = "nonumberofcommits";
//     let mut isssue_label: &str = "isssuelabel";
//     let mut data_hovercard_type: Option<String> = None;
//     let mut data_hovercard_url: Option<String> = None;
//     let mut data_id: Option<String> = None;
//     let mut href: Option<String> = None;
//     let mut data_url: Option<String> = None;
//     let mut the_accounts_repo_on_which_the_action_was_performed_relative_href: Option<String> =
//         None;
//     let mut the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from: Option<
//         String,
//     > = None;
//     let mut from: &str = "nofrom";
//     let mut isssue_label: &str = "isssuelabel";
//     let mut who_follow: &str = "nowhofollow";
//     match option_content {
//         Some(content) => {
//             println!("content{}", content);
//             //             let content = r#""#;
//             let converted_str_to_use_as_document: &str = &content;
//             let document = Document::from(converted_str_to_use_as_document);
//             for node in document.find(Name("span")) {

//             }
//             if let Some(node) = document.find(Class("mb-0").descendant(Name("span"))).next() {
//                 updated_at = Some(node.text())
//             }
//             if let Some(node) = document.find(Name("relative-time")).next() {
//                 if let Some(date_handle) = node.attr("datetime") {
//                     date = Some(date_handle.to_string());
//                 }
//             }
//             let mut vec_of_nodes_a = Vec::<select::node::Node>::with_capacity(6); //change later
//             for node in document.find(Name("a")) {
//                 vec_of_nodes_a.push(node);
//             }
//             // println!("vec_of_nodes_a.len() {}", vec_of_nodes_a.len());
//             let mut vec_of_nodes_with_avatar_link = Vec::<select::node::Node>::with_capacity(4);
//             for node in document.find(Class("avatar-user")) {
//                 vec_of_nodes_with_avatar_link.push(node)
//             }
//             if vec_of_nodes_with_avatar_link.len() == 1 {
//                 if let Some(avatar_link_handle) = vec_of_nodes_with_avatar_link[0].attr("src") {
//                     avatar_link = Some(avatar_link_handle.to_string());
//                 }
//             } else {
//                 println!("todo2 len == {}", vec_of_nodes_with_avatar_link.len())
//             }

//             if vec_of_nodes_a.len() == 4 {
//                 //0 avatar link
//                 //1 user relative link and user name
//                 //2 user's repo relative link and user/repo
//                 //3 user's repo relative link and user/repo
//                 author = Some(vec_of_nodes_a[1].text());
//                 if let Some(author_relative_link_handle) = vec_of_nodes_a[1].attr("href") {
//                     author_relative_link = Some(author_relative_link_handle.to_string());
//                 }
//                 repository_with_author = Some(vec_of_nodes_a[2].text());
//                 if let Some(repository_with_author_relative_link_handle) =
//                     vec_of_nodes_a[2].attr("href")
//                 {
//                     repository_with_author_relative_link =
//                         Some(repository_with_author_relative_link_handle.to_string());
//                 }
//                 // println!("sss{:#?}", vec_of_nodes_a[1].text())
//             } else if vec_of_nodes_a.len() == 6 {
//                 //0 avatar link
//                 //1 user relative link and user name
//                 //2 user's repo relative link and user/repo
//                 //3 user's branch repo relative link and branch/user/repo
//                 //4 avatar link
//                 //5 commit user/repo/commit relative link
//                 author = Some(vec_of_nodes_a[1].text());
//                 if let Some(author_relative_link_handle) = vec_of_nodes_a[1].attr("href") {
//                     author_relative_link = Some(author_relative_link_handle.to_string());
//                 }
//                 repository_with_author = Some(vec_of_nodes_a[2].text());
//                 branch = Some(vec_of_nodes_a[3].text());
//                 if let Some(branch_relative_link_handle) = vec_of_nodes_a[1].attr("href") {
//                     branch_relative_link = Some(branch_relative_link_handle.to_string());
//                 }
//                 if let Some(commit_relative_link_handle) = vec_of_nodes_a[1].attr("href") {
//                     commit_relative_link = Some(commit_relative_link_handle.to_string());
//                 }
//                 // println!("vec_of_nodes_a{:#?}", vec_of_nodes_a)
//             } else {
//                 println!("todo len == {}", vec_of_nodes_a.len());
//             }

//             // let mm = document.find(Name("a")).into_iter().enumerate().len();
//             // match  {
//             //     4 => {}
//             //     _ => {}
//             // }
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
//     println!("avatar_link {:#?}", avatar_link);
//     println!("updated_at {:#?}", updated_at);
//     println!("date{:#?}", date);
//     println!("author{:#?}", author);
//     println!("author_relative_link{:#?}", author_relative_link);
//     println!("repository_with_author{:#?}", repository_with_author);
//     println!(
//         "repository_with_author_relative_link{:#?}",
//         repository_with_author_relative_link
//     );
//     println!("branch{:#?}", branch);
//     println!("branch_relative_link{:#?}", branch_relative_link);
//     println!("commit_relative_link{:#?}", commit_relative_link);
// }

pub fn parse_github_html(option_content: Option<String>) {
    let mut avatar_link: Option<String> = None;
    match option_content {
        Some(content) => {
            let result_of_dom_parse_content = Dom::parse(&content);
            match result_of_dom_parse_content {
                Ok(dom) => {
                    match dom.children.len() {
                        1 => match dom.children[0] {
                            Node::Element(ref dom_first_child) => {
                                match dom_first_child.children.len() {
                                1 => match dom_first_child.children[0] {
                                    Node::Element(ref dom_first_child_first_child) => {
                                        match dom_first_child_first_child.children.len() {
                                            1 => match dom_first_child_first_child.children[0] {
                                                Node::Element(
                                                    ref dom_first_child_first_child_first_child,
                                                ) => {
                                                    match dom_first_child_first_child_first_child.children.len() {
                                                        2 => {
                                                            avatar_link =
                                                                parse_github_html_first_part(
                                                                    &dom_first_child_first_child_first_child.children[0],
                                                                );
                                                            parse_github_html_second_part(
                                                                &dom_first_child_first_child_first_child.children[1],
                                                            );
                                                        }
                                                        _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    dom_first_child_first_child_first_child.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                                    }
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    dom_first_child_first_child.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message =
                                        format!("different children.len(): {}", dom_first_child.children.len());
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                            }
                            _ => print_warning_orange(
                                file!().to_string(),
                                line!().to_string(),
                                "different node".to_string(),
                            ),
                        },
                        _ => {
                            let warning_message =
                                format!("different children.len(): {}", dom.children.len());
                            print_warning_orange(
                                file!().to_string(),
                                line!().to_string(),
                                warning_message,
                            )
                        }
                    }
                }
                Err(e) => {
                    let error_message = format!("Dom::parse error {}", e);
                    print_error_red(file!().to_string(), line!().to_string(), error_message)
                }
            }
        }
        None => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "option content is None".to_string(),
        ),
    }
    println!("avatar_link {:#?}", avatar_link);
}

pub fn parse_github_html_first_part(node: &Node) -> Option<String> {
    let mut avatar_link: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Element(ref node_element_first) => match node_element_first.children.len() {
                    1 => match node_element_first.children[0] {
                        Node::Element(ref node_element_first_first) => {
                            let attribute = "src"; //todo move variable like this into some whole variable structure
                            match node_element_first_first.attributes.get(attribute) {
                                Some(value) => {
                                    avatar_link = value.clone();
                                }
                                None => {
                                    let warning_message = format!("no {} attribute", attribute);
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    },
                    _ => {
                        let warning_message = format!(
                            "different children.len(): {}",
                            node_element_first.children.len()
                        );
                        print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            warning_message,
                        )
                    }
                },
                _ => print_warning_orange(
                    file!().to_string(),
                    line!().to_string(),
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
            }
        },
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    avatar_link
}

pub fn parse_github_html_second_part(node: &Node) {
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
    match node {
        Node::Element(ref node_element) => {
            match node_element.children.len() {
                1 => match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            5 => {
                                match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_first) => {
                                        match node_element_first_first.children.len() {
                                            1 => match node_element_first_first.children[0] {
                                                Node::Text(ref author_handle) => {
                                                    author = author_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_first.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[1] {
                                    Node::Text(ref action_handle) => action = action_handle,
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[2] {
                                    Node::Element(ref node_element_first_third) => {
                                        match node_element_first_third.children.len() {
                                            1 => match node_element_first_third.children[0] {
                                                Node::Text(ref repository_handle) => {
                                                    repository = repository_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_third.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[3] {
                                    Node::Element(ref node_element_first_fourth) => {
                                        match node_element_first_fourth.children.len() {
                                            1 => {
                                                match node_element_first_fourth.children[0] {
                                                    Node::Element(
                                                        ref node_element_first_fourth_first,
                                                    ) => {
                                                        let attribute = "datetime";
                                                        match node_element_first_fourth_first
                                                            .attributes
                                                            .get(attribute)
                                                        {
                                                            Some(value) => {
                                                                datejs = value.clone();
                                                            }
                                                            None => {
                                                                let warning_message = format!(
                                                                    "no {} attribute",
                                                                    attribute
                                                                );
                                                                print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    warning_message,
                                                                )
                                                            }
                                                        }

                                                        match node_element_first_fourth_first
                                                        .children
                                                        .len()
                                                    {
                                                        1 => {
                                                            match node_element_first_fourth_first
                                                                .children[0]
                                                            {
                                                                Node::Text(
                                                                    ref second_child_element5fourth,
                                                                ) => {
                                                                    date =
                                                                        second_child_element5fourth;
                                                                }
                                                                _ => print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    "different node".to_string(),
                                                                ),
                                                            }
                                                        }
                                                        _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_fourth_first.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                                    }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                            }
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_fourth.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[4] {
                                    Node::Element(ref node_element_first_firth) => {
                                        match node_element_first_firth.children.len() {
                                            3 => {
                                                match node_element_first_firth.children[0] {
                                                    Node::Element(
                                                        ref node_element_first_firth_first,
                                                    ) => {
                                                        match node_element_first_firth_first
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_first
                                                                    .children[0]
                                                                {
                                                                    Node::Text(
                                                                        ref actionto_handle,
                                                                    ) => {
                                                                        actionto = actionto_handle;
                                                                    }
                                                                    _ => print_warning_orange(
                                                                        file!().to_string(),
                                                                        line!().to_string(),
                                                                        "different node"
                                                                            .to_string(),
                                                                    ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                    "different children.len(): {}",
                                   node_element_first_firth_first.children.len()
                                );
                                                                print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    warning_message,
                                                                )
                                                            }
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                                match node_element_first_firth.children[1] {
                                                    Node::Element(
                                                        ref node_element_first_firth_second,
                                                    ) => {
                                                        match node_element_first_firth_second
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_second
                                                                    .children[0]
                                                                {
                                                                    Node::Text(ref branch_handle) => {
                                                                        branch = branch_handle;
                                                                    }
                                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                                }
                                                            }
                                                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first_firth_second.children.len()
                                );
                                print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    warning_message,
                                )
                            }
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                                match node_element_first_firth.children[2] {
                                                    Node::Element(
                                                        ref node_element_first_firth_third,
                                                    ) => {
                                                        match node_element_first_firth_third
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_firth_third
                                                                    .children[0]
                                                                {
                                                                    Node::Element(
                                                                        ref
                                                                        node_element_first_firth_third_first,
                                                                    ) => {
                                                                        for i in &node_element_first_firth_third_first.children {
                                                                                parse_github_html_second_part_inner_one_element(&i);
                                                                            }
                                                                    }
                                                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                    "different children.len(): {}",
                                    node_element_first_firth_third.children.len()
                                );
                                                                print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    warning_message,
                                                                )
                                                            }
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                            }
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_firth.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                            }
                            6 => {
                                match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_first) => {
                                        match node_element_first_first.children.len() {
                                            1 => match node_element_first_first.children[0] {
                                                Node::Text(ref author_handle) => {
                                                    author = author_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_first.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[1] {
                                    Node::Element(ref node_element_first_second) => {
                                        match node_element_first_second.children.len() {
                                            1 => match node_element_first_second.children[0] {
                                                Node::Text(ref bot_tag_handle) => {
                                                    bot_tag = bot_tag_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_second.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[2] {
                                    Node::Text(ref action_handle) => action = action_handle,
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[3] {
                                    Node::Element(ref node_element_first_fourth) => {
                                        match node_element_first_fourth.children.len() {
                                            1 => match node_element_first_fourth.children[0] {
                                                Node::Text(ref repository_handle) => {
                                                    repository = repository_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_fourth.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[4] {
                                    Node::Element(ref node_element_first_firth) => {
                                        match node_element_first_firth.children.len() {
                                            1 => match node_element_first_firth.children[0] {
                                                Node::Element(
                                                    ref node_element_first_firth_first,
                                                ) => {
                                                    let attribute = "datetime";
                                                    match node_element_first_firth_first
                                                        .attributes
                                                        .get(attribute)
                                                    {
                                                        Some(value) => {
                                                            datejs = value.clone();
                                                        }
                                                        None => {
                                                            let warning_message = format!(
                                                                "no {} attribute",
                                                                attribute
                                                            );
                                                            print_warning_orange(
                                                                file!().to_string(),
                                                                line!().to_string(),
                                                                warning_message,
                                                            )
                                                        }
                                                    }
                                                    match node_element_first_firth_first
                                                        .children
                                                        .len()
                                                    {
                                                        1 => {
                                                            match node_element_first_firth_first
                                                                .children[0]
                                                            {
                                                                Node::Text(ref date_handle) => {
                                                                    date = date_handle;
                                                                }
                                                                _ => print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    "different node".to_string(),
                                                                ),
                                                            }
                                                        }
                                                        _ => {
                                                            let warning_message = format!(
                                                                "different children.len(): {}",
                                                                node_element_first_firth_first
                                                                    .children
                                                                    .len()
                                                            );
                                                            print_warning_orange(
                                                                file!().to_string(),
                                                                line!().to_string(),
                                                                warning_message,
                                                            )
                                                        }
                                                    }
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_firth.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                                match node_element_first.children[5] {
                                    Node::Element(ref node_element_first_sixth) => {
                                        match node_element_first_sixth.children.len() {
                                            3 => {
                                                match node_element_first_sixth.children[0] {
                                                    Node::Element(
                                                        ref node_element_first_sixth_first,
                                                    ) => {
                                                        match node_element_first_sixth_first
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_sixth_first
                                                                    .children[0]
                                                                {
                                                                    Node::Text(
                                                                        ref actionto_handle,
                                                                    ) => {
                                                                        actionto = actionto_handle;
                                                                    }
                                                                    _ => print_warning_orange(
                                                                        file!().to_string(),
                                                                        line!().to_string(),
                                                                        "different node"
                                                                            .to_string(),
                                                                    ),
                                                                }
                                                            }
                                                            _ => {
                                                                let warning_message =
                                                                    format!(
                                    "different children.len(): {}",
                                    node_element_first_sixth_first.children.len()
                                );
                                                                print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    warning_message,
                                                                )
                                                            }
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                                match node_element_first_sixth.children[1] {
                                                    Node::Element(
                                                        ref node_element_first_sixth_second,
                                                    ) => {
                                                        match node_element_first_sixth_second
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_sixth_second
                                                                    .children[0]
                                                                {
                                                                    Node::Text(ref branch_handle) => {
                                                                        branch = branch_handle;
                                                                    }
                                                                    _ => print_warning_orange(
                                                                        file!().to_string(),
                                                                        line!().to_string(),
                                                                        "different node"
                                                                            .to_string(),
                                                                    ),
                                                                }
                                                            }
                                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_sixth_second.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                                match node_element_first_sixth.children[2] {
                                                    Node::Element(
                                                        ref node_element_first_sixth_third,
                                                    ) => {
                                                        match node_element_first_sixth_third
                                                            .children
                                                            .len()
                                                        {
                                                            1 => {
                                                                match node_element_first_sixth_third
                                                                    .children[0]
                                                                {
                                                                    Node::Element(
                                                                        ref
                                                                        node_element_first_sixth_third_first,
                                                                    ) => {
                                                                        for i in &node_element_first_sixth_third_first.children {
                                                                                parse_github_html_second_part_inner_one_element(&i);
                                                                            }
                                                                    }
                                                                    _ => print_warning_orange(
                                                                        file!().to_string(),
                                                                        line!().to_string(),
                                                                        "different node"
                                                                            .to_string(),
                                                                    ),
                                                                }
                                                            }
                                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_sixth_third.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                            }
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_sixth.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                }
                            }
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_warning_orange(
                        file!().to_string(),
                        line!().to_string(),
                        "different node".to_string(),
                    ),
                },
                2 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_second) => {
                            match node_element_second.children.len() {
                                1 => match node_element_second.children[0] {
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
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                6 => {
                                    match node_element_second.children[0] {
                                        Node::Element(ref node_element_second_first) => {
                                            match node_element_second_first.children.len() {
                                                1 => match node_element_second_first.children[0] {
                                                    Node::Text(ref author_handle) => {
                                                        author = author_handle;
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_first.children.len()
                                                    );
                                                    print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_warning_orange(
                                            file!().to_string(),
                                            line!().to_string(),
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[1] {
                                        Node::Text(ref action_handle) => {
                                            action = action_handle;
                                        }
                                        _ => print_warning_orange(
                                            file!().to_string(),
                                            line!().to_string(),
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[2] {
                                        Node::Element(ref node_element_second_third) => {
                                            match node_element_second_third.children.len() {
                                                1 => match node_element_second_third.children[0] {
                                                    Node::Text(ref release_tag_handle) => {
                                                        release_tag = release_tag_handle;
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_third.children.len()
                                                    );
                                                    print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_warning_orange(
                                            file!().to_string(),
                                            line!().to_string(),
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[3] {
                                        Node::Text(ref of_handle) => {
                                            of = of_handle;
                                        }
                                        _ => print_warning_orange(
                                            file!().to_string(),
                                            line!().to_string(),
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[4] {
                                        Node::Element(ref node_element_second_fourth) => {
                                            match node_element_second_fourth.children.len() {
                                                1 => match node_element_second_fourth.children[0] {
                                                    Node::Text(ref repository_handle) => {
                                                        repository = repository_handle;
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_fourth.children.len()
                                                    );
                                                    print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_warning_orange(
                                            file!().to_string(),
                                            line!().to_string(),
                                            "different node".to_string(),
                                        ),
                                    }
                                    match node_element_second.children[5] {
                                        Node::Element(ref node_element_second_sixth) => {
                                            match node_element_second_sixth.children.len() {
                                                1 => match node_element_second_sixth.children[0] {
                                                    Node::Element(
                                                        ref node_element_second_sixth_first,
                                                    ) => {
                                                        let attribute = "datetime";
                                                        match node_element_second_sixth_first
                                                            .attributes
                                                            .get(attribute)
                                                        {
                                                            Some(datejs_handle) => {
                                                                datejs = datejs_handle.clone();
                                                            }
                                                            None => {
                                                                let warning_message = format!(
                                                                    "no {} attribute",
                                                                    attribute
                                                                );
                                                                print_warning_orange(
                                                                    file!().to_string(),
                                                                    line!().to_string(),
                                                                    warning_message,
                                                                )
                                                            }
                                                        }
                                                        match node_element_second_sixth_first
                                                                .children
                                                                .len()
                                                            {
                                                                1 => {
                                                                    match node_element_second_sixth_first
                                                                .children[0] {
                                                                    Node::Text(ref date_handle) => {
                                                                        date = date_handle;
                                                                    }
                                                                    _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
                                                                }
                                                                }
                                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_sixth_first.children.len()
                                                    );
                                                    print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        warning_message,
                                                    )
                                                }
                                                            }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                },
                                                _ => {
                                                    let warning_message = format!(
                                                        "different children.len(): {}",
                                                        node_element_second_sixth.children.len()
                                                    );
                                                    print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        warning_message,
                                                    )
                                                }
                                            }
                                        }
                                        _ => print_warning_orange(
                                            file!().to_string(),
                                            line!().to_string(),
                                            "different node".to_string(),
                                        ),
                                    }
                                }
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_second.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    //here todo 2
                    println!("2f");
                }
                5 => {
                    println!("5f");
                }
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
                }
            }
        }
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
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

pub fn parse_github_html_second_part_inner_one_element(node: &Node) {
    //out params need to be pushed into vec array or something
    let mut avatar_link: Option<String> = None;
    let mut relative_commit_link: Option<String> = None;
    let mut commit_text: &str = "nocommittext";
    let mut from_text: &str = "nofromtext";
    let mut commits_number: &str = "nonumberofcommits";
    let mut isssue_label: &str = "isssuelabel"; //todo or isssuelabel in struct by default in html
    match node {
        Node::Element(ref node_element) => {
            match node_element.children.len() {
                1 => match node_element.children[0] {
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(ref commits_number_handle) => {
                                    commits_number = commits_number_handle;
                                }
                                _ => print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_warning_orange(
                        file!().to_string(),
                        line!().to_string(),
                        "different node".to_string(),
                    ),
                },
                3 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_first) => {
                                        match node_element_first_first.children.len() {
                                            1 => match node_element_first_first.children[0] {
                                                Node::Element(
                                                    ref node_element_first_first_element,
                                                ) => {
                                                    let attribute = "src";
                                                    match node_element_first_first_element
                                                        .attributes
                                                        .get(attribute)
                                                    {
                                                        Some(avatar_link_handle) => {
                                                            avatar_link =
                                                                avatar_link_handle.clone();
                                                        }
                                                        None => {
                                                            let warning_message = format!(
                                                                "no {} attribute",
                                                                attribute
                                                            );
                                                            print_warning_orange(
                                                                file!().to_string(),
                                                                line!().to_string(),
                                                                warning_message,
                                                            )
                                                        }
                                                    }
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_first.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[1] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_element) => {
                                        let attribute = "href";
                                        match node_element_first_element.attributes.get(attribute) {
                                            Some(relative_commit_link_handle) => {
                                                relative_commit_link =
                                                    relative_commit_link_handle.clone();
                                            }
                                            None => {
                                                let warning_message =
                                                    format!("no {} attribute", attribute);
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[2] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Element(ref node_element_first_first) => {
                                        match node_element_first_first.children.len() {
                                            1 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                            }
                                            2 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                                second_element(
                                                    &node_element_first_first.children[1],
                                                );
                                            }
                                            3 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                                second_element(
                                                    &node_element_first_first.children[1],
                                                );
                                                // println!(
                                                //     "🚀node_element_first_first.children[2]{:#?}",
                                                //     node_element_first_first.children[2]
                                                // );
                                                //here
                                                match node_element_first_first.children[2] {
                                                    Node::Text(ref text) => {
                                                        from_text = text;
                                                    }
                                                    Node::Element(ref something) => {
                                                        if something.name != "g-emoji" {
                                                            println!("todo not g-emoji")
                                                        }
                                                    }
                                                    _ => print_warning_orange(
                                                        file!().to_string(),
                                                        line!().to_string(),
                                                        "different node".to_string(),
                                                    ),
                                                }
                                                //todo
                                                // from_text = handle_text_element(
                                                //     &node_element_first_first.children[2],
                                                // );
                                                //and here
                                            }
                                            5 => {
                                                commit_text = handle_text_element(
                                                    &node_element_first_first.children[0],
                                                );
                                                second_element(
                                                    &node_element_first_first.children[1],
                                                );
                                                from_text = handle_text_element(
                                                    &node_element_first_first.children[2],
                                                );
                                                second_element(
                                                    &node_element_first_first.children[3],
                                                );
                                                handle_text_element(
                                                    //some trash
                                                    &node_element_first_first.children[4],
                                                );
                                            }
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_first_first.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                }
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
                }
            }
        }
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    // println!("avatar_link {:#?}", avatar_link);
    // println!("relative_commit_link {:#?}", relative_commit_link);
    // println!("commit_text {:#?}", commit_text);
    // println!("from_text {:#?}", from_text);
    // println!("commits_number {:#?}", commits_number);
}
pub fn handle_text_element(node: &Node) -> &str {
    let mut text: &str = "";
    match node {
        Node::Text(ref text_handle) => {
            text = text_handle;
        }
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    text
}
pub fn second_element(node: &Node) {
    let mut data_hovercard_type: Option<String> = None;
    let mut data_hovercard_url: Option<String> = None;
    let mut data_id: Option<String> = None;
    let mut href: Option<String> = None;
    let mut data_url: Option<String> = None;
    match node {
        Node::Element(ref node_element) => {
            if let Some(value) = node_element.attributes.get("data-hovercard-type") {
                data_hovercard_type = value.clone();
            }
            if let Some(value) = node_element.attributes.get("data-hovercard-url") {
                data_hovercard_url = value.clone();
            }
            if let Some(value) = node_element.attributes.get("data-id") {
                data_id = value.clone();
            }
            if let Some(value) = node_element.attributes.get("href") {
                href = value.clone();
            }
            if let Some(value) = node_element.attributes.get("data-url") {
                data_url = value.clone();
            }
            //just to check any new formats
            match node_element.children.len() {
                1 => match node_element.children[0] {
                    Node::Text(_) => {
                        //todo what should do with this
                    }
                    Node::Element(ref node_element_first) => {
                        match node_element_first.children.len() {
                            1 => match node_element_first.children[0] {
                                Node::Text(_) => {
                                    //todo what should do with this
                                }
                                _ => print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    "different node".to_string(),
                                ),
                            },
                            _ => {
                                let warning_message = format!(
                                    "different children.len(): {}",
                                    node_element_first.children.len()
                                );
                                print_warning_orange(
                                    file!().to_string(),
                                    line!().to_string(),
                                    warning_message,
                                )
                            }
                        }
                    }
                    _ => print_warning_orange(
                        file!().to_string(),
                        line!().to_string(),
                        "different node".to_string(),
                    ),
                },
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
                }
            }
        }
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    // println!("data_hovercard_type {:#?}", data_hovercard_type);
    // println!("data_hovercard_url {:#?}", data_hovercard_url);
    // println!("data_id {:#?}", data_id);
    // println!("href {:#?}", href);
    // println!("data_url {:#?}", data_url);
}
pub fn two_elements_one_child(node: &Node) {
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
    match node {
        Node::Element(ref node_element) => {
            match node_element.children.len() {
                4 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Text(ref author_name_another_handle) => {
                                        author_name_another = author_name_another_handle;
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[1] {
                        Node::Text(ref action_another_handle) => {
                            action_another = action_another_handle;
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[2] {
                        Node::Element(ref node_element_third) => {
                            let attribute = "href";
                            match node_element_third.attributes.get(attribute) {
                                Some(
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_handle,
                                ) => {
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_handle.clone();
                                }
                                None => {
                                    let warning_message = format!("no {} attribute", attribute);
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[3] {
                        Node::Element(ref node_element_fourth) => {
                            match node_element_fourth.children.len() {
                                1 => match node_element_fourth.children[0] {
                                    Node::Element(ref node_element_fourth_element) => {
                                        let attribute = "datetime";
                                        match node_element_fourth_element.attributes.get(attribute)
                                        {
                                            Some(datejs_another_handle) => {
                                                datejs_another = datejs_another_handle.clone();
                                                //same as in text
                                            }
                                            None => println!("todo15"),
                                        }
                                        match node_element_fourth_element.children.len() {
                                            1 => match node_element_fourth_element.children[0] {
                                                Node::Text(ref date_another_handle) => {
                                                    //same as in datetime attrubute
                                                    date_another = date_another_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_fourth_element.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_fourth.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                }
                5 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Text(ref author_name_another_handle) => {
                                        author_name_another = author_name_another_handle;
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[1] {
                        Node::Text(ref action_another_handle) => {
                            action_another = action_another_handle;
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[2] {
                        Node::Element(ref node_element_third) => {
                            let attribute = "href";
                            match node_element_third.attributes.get(attribute) {
                                Some(
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_handle,
                                ) => {
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_handle.clone();
                                }
                                None => {
                                    let warning_message = format!("no {} attribute", attribute);
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[3] {
                        Node::Text(ref from_handle) => {
                            from = from_handle;
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[4] {
                        Node::Element(ref node_element_firth) => {
                            match node_element_firth.children.len() {
                                1 => match node_element_firth.children[0] {
                                    Node::Element(ref node_element_firth_element) => {
                                        let attribute = "datetime";
                                        match node_element_firth_element.attributes.get(attribute) {
                                            Some(datejs_another_handle) => {
                                                datejs_another = datejs_another_handle.clone();
                                                //same as in text
                                            }
                                            None => println!("todo15"),
                                        }
                                        match node_element_firth_element.children.len() {
                                            1 => match node_element_firth_element.children[0] {
                                                Node::Text(ref date_another_handle) => {
                                                    //same as in datetime attrubute
                                                    date_another = date_another_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_firth_element.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_firth.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                }
                6 => {
                    match node_element.children[0] {
                        Node::Element(ref node_element_first) => {
                            match node_element_first.children.len() {
                                1 => match node_element_first.children[0] {
                                    Node::Text(ref author_name_another_handle) => {
                                        author_name_another = author_name_another_handle;
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_first.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[1] {
                        Node::Text(ref action_another_handle) => {
                            action_another = action_another_handle;
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[2] {
                        Node::Element(ref node_element_third) => {
                            let attribute = "href";
                            match node_element_third.attributes.get(attribute) {
                                Some(value) => {
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href =
                                        value.clone();
                                }
                                None => {
                                    let warning_message = format!("no {} attribute", attribute);
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    );
                                }
                            }
                            match node_element_third.children.len() {
                                1 => match node_element_third.children[0] {
                                    Node::Text(ref isssue_label_handle) => {
                                        isssue_label = isssue_label_handle;
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_third.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[3] {
                        Node::Text(ref from_handle) => {
                            from = from_handle;
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[4] {
                        Node::Element(ref node_element_firth) => {
                            let attribute = "href";
                            match node_element_firth.attributes.get(attribute) {
                                Some(
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle,
                                ) => {
                                    the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from = the_accounts_repo_on_which_the_action_was_performed_relative_href_forked_from_handle.clone();
                                }
                                None => {
                                    let warning_message = format!("no {} attribute", attribute);
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                    match node_element.children[5] {
                        Node::Element(ref node_element_sixth) => {
                            match node_element_sixth.children.len() {
                                1 => match node_element_sixth.children[0] {
                                    Node::Element(ref node_element_sixth_first) => {
                                        let attribute = "datetime";
                                        match node_element_sixth_first.attributes.get(attribute) {
                                            Some(value) => {
                                                datejs_another = value.clone();
                                            }
                                            None => {
                                                let warning_message =
                                                    format!("no {} attribute", attribute);
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                        match node_element_sixth_first.children.len() {
                                            1 => match node_element_sixth_first.children[0] {
                                                Node::Text(ref date_another_handle) => {
                                                    date_another = date_another_handle;
                                                }
                                                _ => print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    "different node".to_string(),
                                                ),
                                            },
                                            _ => {
                                                let warning_message = format!(
                                                    "different children.len(): {}",
                                                    node_element_sixth_first.children.len()
                                                );
                                                print_warning_orange(
                                                    file!().to_string(),
                                                    line!().to_string(),
                                                    warning_message,
                                                )
                                            }
                                        }
                                    }
                                    _ => print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        "different node".to_string(),
                                    ),
                                },
                                _ => {
                                    let warning_message = format!(
                                        "different children.len(): {}",
                                        node_element_sixth.children.len()
                                    );
                                    print_warning_orange(
                                        file!().to_string(),
                                        line!().to_string(),
                                        warning_message,
                                    )
                                }
                            }
                        }
                        _ => print_warning_orange(
                            file!().to_string(),
                            line!().to_string(),
                            "different node".to_string(),
                        ),
                    }
                }
                _ => {
                    let warning_message =
                        format!("different children.len(): {}", node_element.children.len());
                    print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
                }
            }
        }
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
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

pub fn two_elements_four_children_first(node: &Node) {
    let mut author_another_another: &str = "noauthoranotheranother";
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Text(ref author_another_another_handle) => {
                    author_another_another = author_another_another_handle;
                }
                _ => print_warning_orange(
                    file!().to_string(),
                    line!().to_string(),
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
            }
        },
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    // println!("author_another_another {}", author_another_another);
}

pub fn two_elements_four_children_second(node: &Node) {
    let mut action_another_another: &str = "noactionanotheranother";
    match node {
        Node::Text(ref action_another_another_handle) => {
            action_another_another = action_another_another_handle;
        }
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    // println!("action_another_another {}", action_another_another);
}

pub fn two_elements_four_children_third(node: &Node) {
    let mut who_follow: &str = "nowhofollow";
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Text(ref who_follow_handle) => {
                    who_follow = who_follow_handle;
                }
                _ => print_warning_orange(
                    file!().to_string(),
                    line!().to_string(),
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
            }
        },
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    // println!("who_follow {}", who_follow);
}

pub fn two_elements_four_children_fourth(node: &Node) {
    let mut who_follow: Option<String> = None;
    let mut datejs_another: Option<String> = None;
    let mut date_another: Option<String> = None;
    match node {
        Node::Element(ref node_element) => match node_element.children.len() {
            1 => match node_element.children[0] {
                Node::Element(ref node_element_first) => {
                    let attribute = "datetime";
                    let mut datejs_another_handle_attribute: Option<String> = None;
                    let mut datejs_another_handle_text: Option<String> = None;
                    match node_element_first.attributes.get(attribute) {
                        Some(datejs_another_handle) => {
                            println!(
                                "datejs_another_handle_attribute {:#?}",
                                datejs_another_handle_attribute
                            );
                            datejs_another_handle_attribute = datejs_another_handle.clone();
                        }
                        None => {
                            let warning_message = format!("no {} attribute", attribute);
                            print_warning_orange(
                                file!().to_string(),
                                line!().to_string(),
                                warning_message,
                            )
                        }
                    }
                    match node_element_first.children.len() {
                        1 => match node_element_first.children[0] {
                            Node::Text(ref date_another_handle) => {
                                println!(
                                    "datejs_another_handle_text {:#?}",
                                    datejs_another_handle_text
                                );
                                datejs_another_handle_text = Some(date_another_handle.to_string());
                            }
                            _ => print_warning_orange(
                                file!().to_string(),
                                line!().to_string(),
                                "different node".to_string(),
                            ),
                        },
                        _ => {
                            let warning_message = format!(
                                "different children.len(): {}",
                                node_element_first.children.len()
                            );
                            print_warning_orange(
                                file!().to_string(),
                                line!().to_string(),
                                warning_message,
                            )
                        }
                    }
                    // if datejs_another_handle_attribute == datejs_another_handle_text {
                    //     datejs_another = datejs_another_handle_attribute;
                    // } else {
                    //     match datejs_another_handle_attribute {
                    //         Some(ref datejs_attribute) => match datejs_another_handle_text {
                    //             Some(ref datejs_text) => {
                    //                 let warning_message = format!(
                    //             "different between dates: datejs_another_handle_attribute: {:#?}, datejs_another_handle_text: {:#?}",
                    //             datejs_another_handle_attribute,
                    //             datejs_another_handle_text
                    //         );
                    //                 print_warning_orange(
                    //                     file!().to_string(),
                    //                     line!().to_string(),
                    //                     warning_message,
                    //                 );
                    //             }
                    //             None => datejs_another = datejs_another_handle_attribute,
                    //         },
                    //         None => match datejs_another_handle_text {
                    //             Some(ref datejs_text) => {

                    //             }
                    //             None => {},
                    //         }
                    //     }
                    // }
                }
                _ => print_warning_orange(
                    file!().to_string(),
                    line!().to_string(),
                    "different node".to_string(),
                ),
            },
            _ => {
                let warning_message =
                    format!("different children.len(): {}", node_element.children.len());
                print_warning_orange(file!().to_string(), line!().to_string(), warning_message)
            }
        },
        _ => print_warning_orange(
            file!().to_string(),
            line!().to_string(),
            "different node".to_string(),
        ),
    }
    // println!("who_follow {}", who_follow);
    // println!("datejs_another {:#?}", datejs_another);
    // println!("date_another {:#?}", date_another);
}
