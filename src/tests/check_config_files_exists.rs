use std::fs::File;
#[test]
fn check_config_files_exists() {
    let start: &str = "./config/";
    let end: &str = ".toml";
    let vec_of_modes: Vec<&str> = vec!["Default", "Development", "Production", "Testing"];
    for mode in vec_of_modes {
        let file = File::open(format!("{}{}{}", start, mode, end));
        match file {
            Ok(_) => {}
            Err(e) => panic!("file: {} error: {}", mode, e),
        }
    }
}
