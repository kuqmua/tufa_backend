use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn get_git_info() {
    let mut commit_editmsg_content = String::new();
    match File::open(".git/COMMIT_EDITMSG") {
        Err(_) => panic!("no git"),
        Ok(file) => {
            let mut buf_reader = BufReader::new(file);
            match buf_reader.read_to_string(&mut commit_editmsg_content) {
                Ok(_) => {
                    println!("commit_editmsg_content {}", commit_editmsg_content);
                }
                Err(_) => panic!("git cannot read string"),
            }
        }
    }
}
