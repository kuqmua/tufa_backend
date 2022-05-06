# todo_tests

* parsing providers posts tests

* server what running web tests every hour like http requests and get valid data or not
github parsing version to check different structs of posts as test in the loop

### -------------------
Add tests for new libraries versions and what version i use. 
Like http request to crates.io or something similar
<br/>

### -------------------
write test what checks if something missing in .dockerignore and .gitignore - local or ci? 
<br/>

### -------------------
Write tests what checks all function to have #![deny(clippy::indexing_slicing, clippy::unwrap_used)]
<br/>

### -------------------
test to check what cargo run executes in the right folder
<br/>

### -------------------
test what check commit name for only eng symbols
<br/>

### -------------------
#[should_panic(expected = "99")]
write some tests with with macro
<br/>

### -------------------
Right test what parse repo code and check what each function returns some type or enum
<br/>

### -------------------
create big json file to test parsing speed as test
<br/>

### -------------------
test for all env var to working and usage
<br/>

### -------------------
is there some test implementation that checks all your rust project functions (with result return type) on difference between return type in case of ok and error? i mean for cases where size of ok must be >= size of error. if otherwise im going to use Box(Error) in case of error. i think it can be done with big proc macro that expand test.
<br/>

### -------------------
test or proc macro to name error function type like function but with CamelCase and ErrorStruct on the end
<br/>

### -------------------
check all files names to not contain ":" symbol. its needed to correct parsing line
<br/>

### -------------------
check all files name be equal to his inner struct/function
<br/>

### -------------------
try to use nexttest instead of cargo test
https://nexte.st/
https://github.com/nextest-rs/nextest
<br/>

