Status of last deployment: <br/>
<img src="https://github.com/kuqmua/tufa_server/workflows/CI/badge.svg?branch=master"><br/>

# Table of contents
1. [Introduction](#introduction)
2. [Some paragraph](#paragraph1)
    1. [Sub paragraph](#subparagraph1)
3. [Another paragraph](#paragraph2)
4. [simple route request](#simple_route_request)

## This is the introduction <a name="introduction"></a>
Some introduction text, formatted in heading 2 style

## Some paragraph <a name="paragraph1"></a>
The first paragraph text

### Sub paragraph <a name="subparagraph1"></a>
This is a sub paragraph, formatted in heading 3 style

## Another paragraph <a name="paragraph2"></a>
The second paragraph text

# Table of contents
<!-- 1. [simple route request](#simple route request) -->
<!-- 2. [get_providers_posts route request](get_providers_posts route request)
3. [change project config](change project config)
4. [change tests constants](change tests constants)
5. [start docker daemon](start docker daemon)
6. [build docker container (maybe some steps can be ignored)](build docker container (maybe some steps can be ignored)
7. [run docker container](run docker container)
8. [stop docker container](stop docker container)
9. [remove docker container](remove docker container)
10. [remove all unused right now docker containers and images](remove all unused right now docker containers and images)
11. [run containers with docker-compose](run containers with docker-compose)
12. [stop containers with docker-compose](stop containers with docker-compose)
13. [default docker volumes folder on linux](default docker volumes folder on linux)
14. [pull and run mongodb docker container](pull and run mongodb docker container)
15. [start mongodb docker container](start mongodb docker container)
16. [create new rust library](create new rust library)
17. [pull and run postgres docker container](pull and run postgres docker container)
18. [start postres docker container](start postres docker container)
19. [shutdown wsl(if db clients cannot connect to db in wsl2)](shutdown wsl(if db clients cannot connect to db in wsl2)
20. [give priviligies to volumes folder](give priviligies to volumes folder)
21. [start command](start command)
22. [run ci tests](run ci tests)
23. [run local tests](run local tests)
24. [show tree visualization of a dependency graph](show tree visualization of a dependency graph)
25. [how to tune rustfmt](how to tune rustfmt)
26. [check vulnerabilities in project](check vulnerabilities in project)
27. [fix Error: I/O error: Permission denied (os error 13) error](fix Error: I/O error: Permission denied (os error 13) error)
28. [cargo watch](cargo watch)
29. [install custom linker dependencies](install custom linker dependencies)
30. [start deleopment](start deleopment)
31. [pull redis image](pull redis image)
32. [launch Postgres](launch Postgres)
33. [install sqlx-cli](install sqlx-cli)
34. [example add sqlx migration](example add sqlx migration) 
35. [how to use logger](how to use logger)
36. [subscribe route test (change email and name)](subscribe route test (change email and name))
37. [how to install remove unused dependencies tool](how to install remove unused dependencies tool)
38. [how to install logs formatter?](how to install logs formatter?)
39. [docker build](docker build)
40. [Generate query metadata to support offline compile-time verification](Generate query metadata to support offline compile-time verification)
41. [run docker container](run docker container)
42. [smaller rust docker builds](smaller rust docker builds)
43. [ignore Digital Ocean for now](ignore Digital Ocean for now)
44. [ignore How to get started with postmark](ignore How to get started with postmark)
45. [property-based testing](property-based testing)
46. [if tests will be more than 1024](if tests will be more than 1024)
47. [Error: I/O error: Permission denied (os error 13) fix](Error: I/O error: Permission denied (os error 13) fix)
48. [The script needs to be marked as executable and then launched](The script needs to be marked as executable and then launched)
49. [see logs with cargo test](see logs with cargo test)
50. [run integration tests](run integration tests)
51. [run unit tests](run unit tests)
52. [run continious integration tests](run continious integration tests)
53. [links](links) -->

## simple route request <a name="simple_route_request"></a>
```
curl http://127.0.0.1:8080/kekw/index.html
```
### get_providers_posts route request
```
curl http://127.0.0.1:8080/get_providers_posts/
```
### change project config
./env <br/>

### change tests constants
.libs/config_lib/src/get_project_information/project_constans.rs <br/>

### start docker daemon
```
sudo dockerd
```
### build docker container (maybe some steps can be ignored)
```
rustup install nightly
```
```
rustup target add x86_64-unknown-linux-musl
```
```
cargo +nightly build --release
```
```
sudo docker build -t tufa_server-image .
```
### run docker container
```
docker run --env-file .env --name tufa_server-container -p 8000:8000 --rm -it tufa_server-image
```
### stop docker container
```
sudo docker stop tufa_server-container
```
### remove docker container
```
sudo docker rm tufa_server-container
```
### remove all unused right now docker containers and images
```
sudo docker system prune -f 
```
### run containers with docker-compose
```
sudo docker-compose up -d
```
### stop containers with docker-compose
```
sudo docker-compose down
```
### default docker volumes folder on linux
/var/lib/docker/volumes

### pull and run mongodb docker container
(need to write path to your project directory)
```
sudo docker run -p 27017:27017 --name mongo-tufa-wsl2 -v ~/projects/tufa_server/mongodb_volume:/data/db -d mongo:latest
```
### start mongodb docker container
with docker: <br/>
```
sudo docker start mongo-tufa-wsl2
```
with docker-compose(other services too): <br/>
```
sudo docker-compose -f docker-compose.yml up -d 
```
### create new rust library
```
cargo new example_lib --lib
```
### pull and run postgres docker container
```
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest
```
### start postres docker container
with docker: <br/>
```
sudo docker start postgres-tufa-wsl2
```
with docker-compose(other services too): <br/>
```
sudo docker-compose -f docker-compose.yml up -d
```
### shutdown wsl(if db clients cannot connect to db in wsl2)
windows console: <br/>
```
wsl --shutdown
```
then reopen wsl

### give priviligies to volumes folder
```
sudo chown -R username /folderexample 
```
(/db-volumes/mongodb or postgresql) <br/>

### start command
```
cd libs/tests_lib && cargo test local && cd .. && cd .. && cargo run
```
### run ci tests
```
cd libs/tests_lib && cargo test ci -- --show-output
```
### run local tests
```
cd libs/tests_lib && cargo test local -- --show-output
```
### show tree visualization of a dependency graph
```
cargo tree
```
### how to tune rustfmt
You can tune rustfmt for a project with a configuration file, rustfmt.toml. </br> 
Details can be found in [rustfmt’s](https://github.com/rust-lang/rustfmt#configuring-rustfmt)

### check vulnerabilities in project
cargo-audit, a convenient cargo sub-command to check if vulnerabilities have <br/>
been reported for any of the crates in the dependency tree of your project. <br/>
installation: <br/>
```
cargo install cargo-audit
```
usage: <br/>
```
cargo audit
```
### fix Error: I/O error: Permission denied (os error 13) error
```
cd .. sudo chmod -R 777 tufa_server && cd tufa_server
```
### cargo watch
```
cargo watch -x check -x test -x "run | bunyan"
```
### install custom linker dependencies
[page](https://www.lpalmieri.com/posts/session-based-authentication-in-rust/) </br>
On Windows: <br/>
```
cargo install -f cargo-binutils
```
```
rustup component add llvm-tools-preview

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]
```
On Linux: <br/>
Ubuntu  <br/>
```
sudo apt-get install lld clang
```
Arch <br/>
```
sudo pacman -S lld clang
```
```
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]
```
On MacOS <br/>
```
brew install michaeleisel/zld/zld
```
```
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
```
### start deleopment 
```
cargo watch -x check -x test -x "run"
```
### pull redis image
```
sudo docker pull redis
```
### launch Postgres
```
sudo ./scripts/init_db.sh
```
(sudo coz got pesmission denied error)

### install sqlx-cli
```
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres
```
### example add sqlx migration 
```
sqlx migrate add create_subscriptions_table
```
### how to use logger
log provides five macros: trace, debug, info, warn and error.
RUST_LOG=debug cargo run, for example, will surface all logs at debug-level or higher emitted by our application or the crates we are using. RUST_LOG=session_based_authentication, instead, would filter out all records emitted by our dependencies.

### subscribe route test (change email and name)
```
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom'  http://127.0.0.1:8000/subscriptions
```
### how to install remove unused dependencies tool
```
cargo install cargo-udeps
```
usage: <br/>
```
cargo +nightly udeps
```
### how to install logs formatter?
```
cargo install bunyan
```
### docker build 
```
sudo docker build --tag session_based_authentication --file Dockerfile .
```
### Generate query metadata to support offline compile-time verification.
```
sqlx prepare
```
or  <br/>
```
cargo sqlx prepare -- --lib
```
(to use generated .json query data - env var SQLX_OFFLINE must be true)

### run docker container
```
sudo docker run -p 8000:8000 session_based_authentication
```
### smaller rust docker builds
We could go even smaller by using rust:1.59.0-alpine, but we would have to cross-compile to the linux-musl target - out of scope for now. Check out rust-musl-builder if you are interested in generating tiny Docker images.
Another option to reduce the size of our binary further is stripping symbols from it - you can find more information about it here.

### ignore Digital Ocean for now

### ignore How to get started with postmark

### property-based testing
There are two mainstream options for property-based testing in the Rust ecosystem: quickcheck and proptest.

### if tests will be more than 1024
If you have large test suite with a flat file structure, you'll soon be building tens of executable every time you run cargo test. While each executable is compiled in parallel, the linking phase is instead entirely sequential! Bundling all your test cases in a single executable reduces the time spent compiling your test suite in CI3.
If you are running Linux, you might see errors like

thread 'actix-rt:worker' panicked at 
'Can not create Runtime: Os { code: 24, kind: Other, message: "Too many open files" }',

when you run cargo test after the refactoring.
This is due to a limit enforced by the operating system on the maximum number of open file descriptors (including sockets) for each process - given that we are now running all tests as part of a single binary, we might be exceeding it. The limit is usually set to 1024, but you can raise it with ulimit -n X (e.g. ulimit -n 10000) to resolve the issue.

### Error: I/O error: Permission denied (os error 13) fix
```
sudo chown -R $(whoami) session_based_authentication/
```
### The script needs to be marked as executable and then launched:
```
chmod +x ./scripts/init_redis.sh ./script/init_redis.sh
```
### see logs with cargo test
```
cargo test -- --nocapture
```
### run integration tests
```
cargo test integration
```
(integration tests will fail if they run with unit tests)

### run unit tests
```
cargo test unit
```
### run continious integration tests
```
cargo test ci
```
### links
* [How to connect Robo 3T (Robomongo) to MongoDB Atlas (cloud mongoDB database)](https://www.youtube.com/watch?v=t_X7qFMmWhI) </br>
* [pgadmin create table](https://www.youtube.com/watch?v=h5wgbJiSy7Q) </br>
