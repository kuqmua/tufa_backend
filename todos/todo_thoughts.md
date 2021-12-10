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
Move all from user_credentials into .env file
<br/>

### -------------------
config as dynamic library maybe?
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
clippy rust flags (can use deny for them - code would be more safe)
https://efanzh.org/2021/05/25/rust-lints.html
<br/>

### -------------------
global .gitignore
https://www.youtube.com/watch?v=D97rnxDqq1I
<br/>

### -------------------
#[deny(clippy::unwrap_used)]
Find out why its triggers for tokio:main functions
<br/>

### -------------------
rewrite something with "from" and "into" traits
<br/>

### -------------------
check rust-analyzer flags to enable
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
find out dofference between diesel vs tokio-postgres vs sqlx
<br/>

### -------------------
find out how tokio runtime works if there is no join all method and will it be actuall in parallel or not
<br/>


### -------------------
Compile on save
cargo watch -q -c -x run 
<br/>
    
### -------------------
add stackoverflow provider
https://stackoverflow.com/feeds/question/23412033
<br/>
    
### -------------------
difference between usage in global scope and local scope
use lib_or_crate_or_something::something;
<br/>

### -------------------
https://github.com/tangramdotdev/tangram
train, deploy, and monitor machine learning models
<br/>

### -------------------
Add different project in inner folder of tufa backend 
what parse few rust files, dockerfile, docker-compose and .env 
And add new env variable and logic around it in all that files
Then run tests and cargo check
<br/>
    
### -------------------
pull requests instead of commits (+ github actions) - do not add changes if test fail
<br/>
    
### -------------------
move from ubuntu to alphine linux on wsl2
<br/>
    
### -------------------
if i go into docker container and modify file, will changes be saved after container restart?
if yes then maybe write some send logs logic around it?
like check last logs send date and time then decide send or not send
<br/>

### -------------------
add to dockerfile or before docker build test what check valid env data fields
<br/>

### -------------------
pm2 instead of docker?
<br/>
    
### -------------------
commit message and hash in file inside docker container
for more information about code inside container
<br/>
    
### -------------------
red hat / openshift / reg.ru postgres cloud cluster
<br/>

### -------------------
rename ERROR_RED="255" # u8 into ERROR_PRINT_COLOR_RED and others
<br/>

### -------------------
rust language detection library
https://github.com/pemistahl/lingua-rs
<br/>

### -------------------
https://doc.rust-lang.org/std/macro.stringify.html
<br/>

### -------------------
find some "try to remove .clone() from rust code" exercises
<br/>

### -------------------
This week in rust check 
<br/>

### -------------------
Hazard pointer https://youtu.be/2Iu2BnO9iHg
<br/>

### -------------------
Type safe relationship between enums.
We have two different enums Number and ErrorNumber
And we can write some matching for them
Like Number::One => ErrorNumber::One
This can be realized safetly as tree
Like implement get_error for Number or something
It can be graph but we can mismatch something like
Number::One => ErrorNumber::One
Number::Two => ErrorNumber::One
Its not a  TYPE SAFE Graph, coz it's compiled successfully
It must be many to many Graph-enum-type-system
Ask someone about it
<br/>

### -------------------
find out what happens in function can return some error
but you use ? operator on result and await cases
<br/>

### -------------------
Complete all exercises
https://github.com/rust-lang/rustlings
<br/>

### -------------------
learn more about iterator methods

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.into_iter().filter(|pair| pair.1 == &value).count()
    --------------
    WHY THIS NOT WORKING?
    map.into_iter().filter(|(key, val)| val == &value).count()
}
fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.into_iter().fold(0, |result, map| {
        result + map.into_iter().filter(|pair| pair.1 == &value).count()
    })
}
<br/>

### -------------------
learn more about rust macro system
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!")
    };
}
<br/>

### -------------------
AsRef trait find out more
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}
<br/>

### -------------------
error conversion with ? example

use std::num::ParseIntError;
use std::str::FromStr;
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}
impl From<CreationError> for ParsePosNonzeroError {
    fn from(e: CreationError) -> Self {
        ParsePosNonzeroError::Creation(e)
    }
}
impl From<ParseIntError> for ParsePosNonzeroError {
    fn from(e: ParseIntError) -> Self {
        ParsePosNonzeroError::ParseInt(e)
    }
}
impl FromStr for PositiveNonzeroInteger {
    type Err = ParsePosNonzeroError;
    fn from_str(s: &str) -> Result<PositiveNonzeroInteger, Self::Err> {
        let x: i64 = s.parse()?;
        Ok(PositiveNonzeroInteger::new(x)?)
    }
}
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);
#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            PositiveNonzeroInteger::from_str("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }
    #[test]
    fn test_negative() {
        assert_eq!(
            PositiveNonzeroInteger::from_str("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }
    #[test]
    fn test_zero() {
        assert_eq!(
            PositiveNonzeroInteger::from_str("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }
    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(PositiveNonzeroInteger::from_str("42"), Ok(x.unwrap()));
    }
}
<br/>

### -------------------
find out more about downcast errors
<br/>

### -------------------
.map_err() function in code examples
<br/>

### -------------------
use Option ref to remove borrow error
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
<br/>
    
### -------------------
rewrite error handling like this?
    if let Err(merge_errors) = builder.merge_pipelines() {
        errors.extend(merge_errors);
    }
<br/>


### -------------------
how to clone custom error? which contains std error or lib errors
<br/>

### -------------------
crate for git repos integration 
git2 = "0.13"
<br/>
    
### -------------------
impl serde::Serialize and deserialize for structs and enums
<br/>

### -------------------
learn https://github.com/Amanieu/parking_lot
<br/>
    
### -------------------
systemclt is-enabled docker
<br/>
    
### -------------------
read about async cancelation
https://blog.yoshuawuyts.com/async-cancellation-1/
<br/>

### -------------------
read about debugging Rust application inside linux container
https://blog.erebe.dev/blog/debug-rust-aplication-inside-container/index.html
<br/>

### -------------------
read how to write async tests
https://docs.rs/tokio/1.14.0/tokio/attr.test.html
<br/>
  
### -------------------
Checking Unused Dependencies in a Rust Project with Github Actions
https://erayerdin.com/checking-unused-dependencies-in-a-rust-project-with-github-actions-ckwm3yov901cwlvs1h48z54xi
<br/>
    
    

