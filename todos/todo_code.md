# todo_code

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
rename some local variables in functions
<br/>

### -------------------
service for date/time checking and executing arxiv for example one time per week and check of server restarted in this timestamp
<br/>

### -------------------
problem - now code waiting for all http reqwests to complete. rewrite it into event loop
Or message queue
<br/>

### -------------------
there is a problem with main arxiv link to check if provider available or not. and not only with arxiv
<br/>

### -------------------
do some work on better differences in colors in prints
<br/>

### -------------------
function to write save path from string and change some symbols
(Partially done)
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
Or tokio tasks
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
generate_biorxiv_hashmap_links and others rename this - remove hashmap
<br/>

### -------------------
thread pool with this let cpus = num_cpus::get();
<br/>

### -------------------
write some logic and flag what choose between env and in code constants for more efficient production variant 
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
rename this check_new_posts_threads_parts
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
if cannot connect to db then program not ending. just waiting. find out why and fix it
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
make default value for all environment variables
I DONT THINK ITS A GOOD IDEA
<br/>
            
### -------------------
dockerfile with ARM support
<br/>

### -------------------
rename MONGO_FOURTH_HANDLE_URL_PART and others into something more meaningfull 
<br/>

### -------------------
vec_of_values are not filtered in
mongo_insert_docs_in_empty_collection(
        mongo_url,
        db_name_handle,
        db_collection_handle, //fix naming later
        db_collection_document_field_name_handle,
        vec_of_values,
    )
all 2200+ twitter subs inserted
fix this by adding filter from env
<br/>

### -------------------
early return refactoring with ok_or and ?
example
fn bar1(x: Option<u64>) -> Result<u64, MyErrors> {
    let x = x.ok_or(MyErrors::SomeError)?;
    // A lot of stuff going on.
    Ok(x * 2)
}
<br/>
            
### -------------------
optimize for loop for std::env::var to std::env::vars
but will it be safe?
<br/>

### -------------------
env vars names - make it private
<br/>

### -------------------
seaORM instead of diesel?
https://www.sea-ql.org/SeaORM/
Diesel	SeaORM
Sync	Async
Static	Dynamic
Native Driver	Pure Rust
Relational
Schema first
MySQL / Postgres / SQLite

NOPE, NOT STABLE YET
check it in the middle of 2022
<br/>

### -------------------
restructure config with prints Structure fields (together some inner structs)
<br/>
            
### -------------------
remove unwrap() into expect() to give more meaning
except maybe loops or early return cases 
<br/>

### -------------------
warning: variant is never constructed: `PostgreSql`
  --> src/providers_info/get_project_information/get_providers_link_parts.rs:62:5
   |
62 |     PostgreSql,
   |     ^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default
<br/>

### -------------------
Result instead of option in all code possible coz return Option loosing information about error 
<br/>

### -------------------
big enum of enums(recursive) of all errors in project
<br/>

### -------------------
move providers link parts (specific urls) to project constants
<br/>

### -------------------
remove unhandled_success_handled_success_are_there_items_initialized_posts_dir - like fileds from config
rename it 
<br/>

### -------------------
if error size > then ok return type then allocate error on heap, otherwise actual return type would be more then needed
maybe write some tests or linter or lib or rustanalyzer flag to check it
<br/>

### -------------------
Rename all functions return type to mean something.
type = ....
<br/>

### -------------------
wrap blocking diesel function into spawn blocking under tokio runtime
(rustacean station - tokio ecosystem with alice ryhl) 24:30
<br/>

### -------------------
Common projects constants as lazy static to reuse
Them inside parent modules
<br/>
  
### -------------------
procedural macros for config fields like vec![]. with correct naming require
without typed enums
like 
Config {
  test_field: from_env!(test_field.to_upper_snake_case()) 
}
<br/>

### -------------------
reuse files with functions and constants using #[path=""] instead of module system
its maybe bad practice but must work
<br/>
