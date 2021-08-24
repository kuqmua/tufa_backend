# todo_thoughts

### -------------------
`iter()` for vecs yields `&i32`.
let mut iter = vec1.iter();
`into_iter()` for vecs yields `i32`.
let mut into_iter = vec2.into_iter();

`iter()` for vecs yields `&i32`, and we want to reference one of its
items, so we have to destructure `&&i32` to `i32`
println!("Find 2 in vec1: {:?}", iter .find(|&&x| x == 2));
`into_iter()` for vecs yields `i32`, and we want to reference one of
its items, so we have to destructure `&i32` to `i32`
println!("Find 2 in vec2: {:?}", into_iter.find(| &x| x == 2));
<br/>

### -------------------
extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
<br/>

### -------------------
for loop into iterators?
<br/>

### -------------------
add everywhere explicit types
<br/>

### -------------------
std::stringify
<br/>

### -------------------
std::str::SplitN
<br/>

### -------------------
split().map()
instead of find(<item></item>)
<br/>

### -------------------
error had .description method
maybe rewrite error messages?
<br/>

### -------------------
hashmaps into vecs
<br/>

### -------------------
&str instead of String everywhere if possible
<br/>

### -------------------
'static remove if possible or maybe add if its better 
<br/>

### -------------------
ensure! macros
<br/>

### -------------------
RES.STATUS: 521 <unknown status code>
<br/>

### -------------------
operation timed out print time elapsed
<br/>

### -------------------
Rc<RefCell<Data>>
Rc::new(<RefCell::new(Data { value: 42}))
<br/>

### -------------------
find out why constants which used in tests looks like unused
<br/>

### -------------------
google some style guide variable naming for project(parsing espesially) and rename variables
<br/>

### -------------------
[dependencies] and [dev-dependencies] read about it
<br/>

### -------------------
impl UserCredentialsStruct {
    pub fn new() -> Result<Self, ConfigError> {
        maybe add different user logic later ?
<br/>

### -------------------
why hashmap['key'] do not return Result ?????7  maybe switch to stable instead of nightly
<br/>

### -------------------
thread '<unnamed>' panicked at 'twitter_provider_names is empty!!!', libs/providers_info_lib/src/get_project_information/generate_hashmap_links/generate_twitter_hashmap_links.rs:7:9
+++++++++++++++++++++++++++
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/tufa_backend`
We are on a multicore system with 12 CPUs
ENV: Development
server can reach https://www.google.com/
20 elements in Twitter HashMap
i can reach https://www.google.com/
file: src/fetch/rss_check_available_providers.rs:32
UnhandledFetchStatusInfo::Failureerror sending request for url (https://nitter.fdn.fr/TheCherno/rss): error trying to connect: tcp connect error: Network is unreachable (os error 101)
thread '<unnamed>' panicked at 'twitter_provider_names is empty!!!', libs/providers_info_lib/src/get_project_information/generate_hashmap_links/generate_twitter_hashmap_links.rs:7:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Any', src/check_new_posts_threads_parts.rs:418:18ed
<br/>

### -------------------
parts of config as feature flags? 
<br/>

### -------------------
is there some service whats run postgres and mongo in cloud for free like 100mb for free?
<br/>

### -------------------
Dynamic library (dylib)
Find out about it 
crate-type = ["dylib"]
<br/>

### -------------------
difference between "".to_string() and String::new()
<br/>

### -------------------
refactor project into rust workspaces to reuse dependencies
<br/>

### -------------------
caret requirements 
tilde requirements
wildcard requirements
<br/>

### -------------------
different user_credentials files for project databases and providers
<br/>

### -------------------
config as dynamic library maybe?
<br/>

### -------------------
Restructure config for providers like this
arxiv_params = { name: "arxiv", enable_arxiv: true, something else...}
<br/>

### -------------------
Faster docker build 
Cargo chef
https://github.com/LukeMathWalker/cargo-chef
<br/>

### -------------------
Redis rust library
https://github.com/mitsuhiko/redis-rs
<br/>

### -------------------
AVIF imag format in some cases better than webp, png and jpeg
<br/>

### -------------------
todo files folder
<br/>

### -------------------
clippy rust flags (can use deny for them - code would be more safe)
https://efanzh.org/2021/05/25/rust-lints.html
<br/>

### -------------------
global .gitignore
https://www.youtube.com/watch?v=D97rnxDqq1I
<br/>

### -------------------
#[deny(clippy::unwrap_used)]
is it triggers for all async functions? or only for tokio?
<br/>

### -------------------
rewrite something with "from" and "into" traits
<br/>

### -------------------
check rust-analyzer flags to enable
<br/>

### -------------------
reorganize config to toml objects(key value)?
https://parsetoml.readthedocs.io/en/latest/access.html
<br/>
    
### -------------------
"Rust Doc Viewer" vscode extension
<br/>

### -------------------
check if i have dependency with std::net in code. (vulnerability in std::net), after Rust 1.53.0 no problem with that
<br/>
    
### -------------------
find out how to use ngnix
<br/>

### -------------------
add to each function return bool if its not returning yet
<br/>

### -------------------
how to solve problem with two different versions of tokio runtime? main 1.7.1, for mongo 0.2.25
<br/>

### -------------------
find out dofference between diesel vs tokio-postgres
<br/>

### -------------------
find out how tokio runtime works if there is no join all method and will it be actuall in parallel or not
<br/>


### -------------------
Compile on save
cargo watch -q -c -x run 
<br/>
