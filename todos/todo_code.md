# todo_code

### -------------------
db for provider links
<br/>

### -------------------
write logs into db in different service
<br/>

### -------------------
add write logs in db into dir if provider doesnt respond to request 
<br/>

### -------------------
write fetching result in file and check if there same posts or not
<br/>

### -------------------
providers statistics in files. if 3 time provider is inactive then do not fetch him next time then next two times...
<br/>

### -------------------
check every link's status code - maybe they rename it or something
only status without body
<br/>

### -------------------
do http reqwest again for some unsuccess http statuses in vec of unsuccess links
reqwest::StatusCode::REQUEST_TIMEOUT 
let wrong_cases_thread = thread::spawn(move || {
refetch logic
<br/>

### -------------------
compute post's hash and send hashes from client app to server to check server already send them or not 
<br/>

### -------------------
items: vec![BiorxivPageStructItem::new(); 30],
whats strange what only 30...weird
<br/>

### -------------------
rename some local variables in functions
<br/>

### -------------------
service for date/time checking and executing arxiv for example one time per week and check of server restarted in this timestamp
<br/>

### -------------------
problem - now code waiting for all http reqwests to complete. rewrite it into event loop
<br/>

### -------------------
there is a problem with main arxiv link to check if provider available or not. and not only with arxiv
<br/>

### -------------------
do some work on better differences in colors in prints
<br/>

### -------------------
function to write save path from string and change some symbols
<br/>

### -------------------
if size of working dir > 100mb then remove all containg
<br/>

### -------------------
write analog for twitter an use not only nitters
404 Not Found - CAUSE
This account's tweets are protected.
Only confirmed followers have access to @_KudoHiroyuki's tweets.
<br/>

### -------------------
https://doc.rust-lang.org/std/primitive.i32.html#method.checked_add
rewrite code in which there is a buffer overflow
<br/>

### -------------------
futures in some cases instead of threads (like file open or write in file)
<br/>

### -------------------
thread pool instead of for loop
<br/>

### -------------------
let _ = join_all(vec_of_write_into_files_futures).await; //todo: add state of success/unsuccess
<br/>

### -------------------
if let Ok(something) = something.lock() {}
instead of 
something.lock().unwrap();
or match some none
<br/>

### -------------------
create big json file to test parsing speed
<br/>

### -------------------
generate_biorxiv_hashmap_links and others rename this - remove hashmap
<br/>

### -------------------
thread pool with this let cpus = num_cpus::get();
<br/>

### -------------------
write some logic and flag what choose between config values, env and in code constants for more efficient production variant 
<br/>

### -------------------
pub const PROJECT_MODE: &str = "Development";//later as ENV variable only
<br/>

### -------------------
todo: add medium
<br/>

### -------------------
handle all todos in parse_github_html
<br/>

### pull and run postgres docker container
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest

how to change default volume folder for this command?
and mongo
and in dcoker-compose too
<br/>

### -------------------
find this in code and fix "(todo change this print)" 
<br/>

### -------------------
limit for mongo "get data" functions
get concrete number of provider links as function or command line or env
<br/>

### -------------------
#[deny(clippy::indexing_slicing, clippy::unwrap_used)]
files to write warnings:
rss_parse_string_into_struct
rss_fetch_and_parse_provider_data
rss_divide_to_equal_for_each_provider
rss_check_available_providers
parse_github_html and inner
get_providers_link_parts_from_mongo
generate_twitter_hashmap_links
<br/>

### -------------------
github parsing return in second parameter option<String, line! file!> to analize missing parse logic 
<br/>

### -------------------
implement get_providers_link_parts with all success completed and with not all
<br/>

### -------------------
get_providers_link_parts whole and for each provider
and maybe rewrite it as struct with methods
<br/>

### -------------------
Resource::PostgreSql => { 

<br/>

### -------------------
let mut vec_of_enums: Vec<ProviderKind> =
            Vec::with_capacity(CONFIG.params.vec_of_provider_names.len());
        //check if provider_names are unique
<br/>

### -------------------
rename this check_new_posts_threads_parts
<br/>

### -------------------
Struct itertools::structs::Unique
//check if provider_names are unique
            for provider_name in &CONFIG.params.vec_of_provider_names {
<br/>

### -------------------
file: src/fetch/parse_github_html.rs:1226
different children.len(): 3
file: src/fetch/parse_github_html.rs:1553
different children.len(): 3
<br/>

### -------------------
5 => {
                println!("todo 5 elements github parsing")
            }
github parsing
<br/>

### -------------------
add provider_kind into inner related to providers functions as input parameter
<br/>


### -------------------
restruct config to remove this 
[enable_providers]
enable_arxiv = true
enable_biorxiv = true
enable_github = true
enable_habr = false
enable_medrxiv = false
enable_reddit = false
enable_twitter = false

into this 
vec_of_provider_names = ["arxiv", "biorxiv", "github", "habr", "medrxiv", "reddit", "twitter"]

as {
  name: "biorxiv",
  enable: true,
  enable_warning_high_prints: true,
}
<br/>

### -------------------
pub enum PrintType {
    Error,
    WarningHigh,
    WarningLow,
    Success,
    PartialSuccess,
    TimeMeasurement,
    CleaningWarningLogsDirectory,
}
add Info
add ThreadError
and other specific errors
<br/>

### -------------------
twitter fetch Syntax: 2:97 Element atom:link prefix is unbound
<br/>

### -------------------
ProviderKind::Medrxiv => {
                            fetch_result_string.remove(0);

rewrite it without extra allocation. like let f = fetch_result_string[1..]
<br/>

### -------------------
rewrite ProviderKind input params as function params in get_providers_link_parts_from_mongo
<br/>
            
### -------------------
get links_limit in random order not first 10
<br/>

### -------------------
check config values on non negative, overflow, more than capacity, not zero
<br/>

### -------------------
Add method to config struct what check integer value for correct value 
and use this method instead of use fields directly. 
Make fields private and use get/set. Check strings too
<br/>

### -------------------
//todo write into mongo collection and create flag where to write logs
rss_write_error_logs_into_file_for_provider(file_name, json_object);
<br/>

### -------------------
add bool or option return type to create_dir_if_dont_exists. maybe it would be runtime error if i dont
<br/>

### -------------------
std::path::PathBuf instead if string in path to file logic
<br/>

### -------------------
if cannot connect to db then program do not ending. just waiting. find out why and fix it
<br/>

### -------------------
add area of visibility / scope like this for all private functions/methonds
pub(in crate::get_project_information::get_config::structures_implementations::config_struct_impl)
<br/>

### -------------------
//todo: find out why cannot write this path crate::get_project_information::get_config::structures_implementations::config_struct_impl::wrap_config_checks_impl
<br/>

### -------------------
https://github.com/rust-cv
rust image detection libs/crates
<br/>

### -------------------
maybe add this to code
#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
<br/>

### -------------------
match config_set_env_ok.merge(File::with_name(&format!(

remove opening file
remove config_lib(from crates.io)
<br/>

### -------------------
make default value for all environment variables
<br/>