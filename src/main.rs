/*
#[path = "providers/initialization/reddit_part.rs"]
mod reddit_part;
use reddit_part::reddit_part;

fn main() {
    reddit_part();
}
*/

extern crate reqwest;
extern crate xml;

use quick_xml::events::Event;
use quick_xml::Reader;

#[path = "providers/parsing/arxiv/arxiv_xml_structs/arxiv_post.rs"]
mod arxiv_post;
use arxiv_post::ArxivPost;
use arxiv_post::Creator;
//math.QA
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://arxiv.org/rss/math.QA") //?mirror=ru
        .await?
        .text()
        .await?;
    let xml: String = resp;
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    let mut count = 0;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"tag1" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                b"tag2" => count += 1,
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("{}", count);
    let mut remove_element = 0;
    while remove_element < 13 {
        txt.remove(0);
        remove_element += 1;
    }

    let mut vec_of_arxiv_posts: Vec<ArxivPost> = Vec::new();
    let mut write_count = 0;
    while write_count < txt.len() {
        let mut arxiv_post: ArxivPost = ArxivPost::new();
        let start_title = txt[write_count].clone();
        if let Some(index) = start_title.find(". (arXiv") {
            arxiv_post.title = start_title[..index].to_string();
        } else {
            arxiv_post.title = start_title;
            println!("whole title written and start.find(. (arXiv) works uncorrectly");
        }

        arxiv_post.link = txt[write_count + 1].clone();
        let description_start = txt[write_count + 2].clone();
        if let Some(index_from_start) = description_start.find("<p>") {
            if let Some(index_from_end) = description_start.find("\n</p>") {
                arxiv_post.description =
                    description_start[index_from_start + "<p>".len()..index_from_end].to_string();
            } else {
                arxiv_post.description =
                    description_start[index_from_start + "<p>".len()..].to_string();
                println!(
                    "error to find index_from_end, but index_from_start was found, write_count={}",
                    write_count
                );
            }
        } else {
            if let Some(index_from_end) = description_start.find(" </p>") {
                arxiv_post.description = description_start[..index_from_end].to_string();
                println!(
                    "error to find index_from_start, but index_from_end was found, write_count={}",
                    write_count
                );
            } else {
                arxiv_post.description = description_start;
                println!("whole title written and description_start.find() worked uncorrectly, write_count={}", write_count);
            }
        }
        let creators_string = txt[write_count + 3].clone();
        let mut string_part_for_loop = creators_string;
        let mut creators_count = 0;
        while let Some(link_index_from_start) = string_part_for_loop.find("<a href=\"") {
            if creators_count > 256 {
                panic!("why creators_count > 256?"); //переписать когда примешь стандарт по которому ошибки обрабатывать будешь
            } else {
                if let Some(link_index_from_end) = string_part_for_loop.find("\">") {
                    if let Some(name_index_from_end) = string_part_for_loop.find("</a>") {
                        let mut creator = Creator::new();
                        creator.link = string_part_for_loop[link_index_from_start + "<a href=\"".len()..link_index_from_end].to_string();
                        let name_index_from_start = link_index_from_end + "\">".len();//это же за два символа считается да в длине ?
                        println!("creator.link = {}", creator.link);
                        println!("string_part_for_loop.len() = {}", string_part_for_loop.len());
                        println!("name_index_from_start = {}", name_index_from_start);
                        println!("name_index_from_end = {}", name_index_from_end);
                        //creator.name = string_part_for_loop[name_index_from_start + "\">".len()..name_index_from_end].to_string();
                        creator.name = string_part_for_loop[name_index_from_start..name_index_from_end].to_string();
                        println!("creator.name = {}", creator.name);
                        arxiv_post.creators.push(creator);
                        string_part_for_loop = string_part_for_loop[name_index_from_end + "</a>".len()..].to_string();
                        creators_count += 1;
                    } else {
                        let mut creator = Creator::new();
                        creator.link = string_part_for_loop[link_index_from_start + "<a href=\"".len()..link_index_from_end].to_string();
                        creators_count += 1;
                        println!(
                    "error to find name_index_from_end, but name_index_from_start(what equals to link_index_from_end + its len) was found, Creator pushed with default creator.name, creators_count={}",
                    creators_count
                );
                    }
                } else {
                    creators_count += 1;//надо ли тут инкрементить?
                    println!(
                    "error to find link_index_from_end, but link_index_from_start was found, creators_count={}",
                    creators_count      
                );
                break
                }
            }
            println!("end of iteration");
        }
        vec_of_arxiv_posts.push(arxiv_post);
        write_count += 4;
    }
    println!("{}", write_count);
    println!("{:#?}", vec_of_arxiv_posts[0].creators[0].name);

    Ok(())
}
