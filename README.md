Status of last deployment: <br/>
<img src="https://github.com/kuqmua/tufa_server/workflows/CI/badge.svg?branch=master"><br/>

### simple route request
curl http://127.0.0.1:8080/kekw/index.html

### get_providers_posts route request
curl http://127.0.0.1:8080/get_providers_posts/

### change project config
./env

### change tests constants
.libs/config_lib/src/get_project_information/project_constans.rs <br/>

### start docker daemon
sudo dockerd

### build docker container
(maybe?) rustup install nightly
(maybe?) rustup target add x86_64-unknown-linux-musl
(maybe?) cargo +nightly build --release
sudo docker build -t tufa_server-image .

### run docker container
docker run --env-file .env --name tufa_server-container -p 8000:8000 --rm -it tufa_server-image

### stop docker container
sudo docker stop tufa_server-container

### remove docker container
sudo docker rm tufa_server-container

### remove all unused right now docker containers and images
sudo docker system prune -f 

### run containers with docker-compose
sudo docker-compose up -d

### stop containers with docker-compose
sudo docker-compose down

### default docker volumes folder on linux
/var/lib/docker/volumes

### pull and run mongodb docker container
(need to write path to your project directory)
sudo docker run -p 27017:27017 --name mongo-tufa-wsl2 -v ~/projects/tufa_server/mongodb_volume:/data/db -d mongo:latest

### How to connect Robo 3T (Robomongo) to MongoDB Atlas (cloud mongoDB database)
https://www.youtube.com/watch?v=t_X7qFMmWhI

### start mongodb docker container
with docker: sudo docker start mongo-tufa-wsl2 <br/>
with docker-compose(other services too): sudo docker-compose -f docker-compose.yml up -d 

### create new rust library
cargo new example_lib --lib

### pull and run postgres docker container
sudo docker run -p 5432:5432/tcp --name postgres-tufa-wsl2 -v ~/db-volumes/postgresql-volumes/tufa-dev-volume -e POSTGRES_PASSWORD=postgres -d postgres:latest

### start postres docker container
with docker: sudo docker start postgres-tufa-wsl2 <br/>
with docker-compose(other services too): sudo docker-compose -f docker-compose.yml up -d <br/>

### postgres create table
https://www.youtube.com/watch?v=h5wgbJiSy7Q

### shutdown wsl(if db clients cannot connect to db in wsl2)
windows console: wsl --shutdown <br/>
then reopen wsl

### give priviligies to volumes folder
sudo chown -R username /folderexample <br/>
(/db-volumes/mongodb or postgresql)

### start command
cd libs/tests_lib && cargo test local && cd .. && cd .. && cargo run

### run ci tests
cd libs/tests_lib && cargo test ci -- --show-output

### run local tests
cd libs/tests_lib && cargo test local -- --show-output

### show tree visualization of a dependency graph
cargo tree

### how to tune rustfmt
You can tune rustfmt for a project with a configuration file, rustfmt.toml. Details can be found in
rustfmt’s https://github.com/rust-lang/rustfmt#configuring-rustfmt

### check vulnerabilities in project
cargo-audit, a convenient cargo sub-command to check if vulnerabilities have
been reported for any of the crates in the dependency tree of your project.
installation:
cargo install cargo-audit
usage:
cargo audit

### fix Error: I/O error: Permission denied (os error 13) error
cd ..
sudo chmod -R 777 tufa_server

### cargo watch 
cargo watch -x check -x test -x "run | bunyan"

### install custom linker dependencies
page https://www.lpalmieri.com/posts/session-based-authentication-in-rust/l
On Windows 
cargo install -f cargo-binutils
rustup component add llvm-tools-preview

[target.x86_64-pc-windows-msvc]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.x86_64-pc-windows-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

On Linux:
Ubuntu, `sudo apt-get install lld clang`
Arch, `sudo pacman -S lld clang`
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

### start deleopment 
cargo watch -x check -x test -x "run"

### pull redis image
sudo docker pull redis

### launch Postgres
sudo ./scripts/init_db.sh
(sudo coz got pesmission denied error)

### install sqlx-cli
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

### example add sqlx migration 
sqlx migrate add create_subscriptions_table

### how to use logger
log provides five macros: trace, debug, info, warn and error.
RUST_LOG=debug cargo run, for example, will surface all logs at debug-level or higher emitted by our application or the crates we are using. RUST_LOG=session_based_authentication, instead, would filter out all records emitted by our dependencies.

### subscribe route test (change email and name)
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom'  http://127.0.0.1:8000/subscriptions

### how to install remove unused dependencies tool
cargo install cargo-udeps
usage:
cargo +nightly udeps

### how to install logs formatter?
cargo install bunyan

### docker build 
sudo docker build --tag session_based_authentication --file Dockerfile .

### Generate query metadata to support offline compile-time verification.
sqlx prepare
or 
cargo sqlx prepare -- --lib
(to use generated .json query data - env var SQLX_OFFLINE must be true)

### run docker container
sudo docker run -p 8000:8000 session_based_authentication

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
/////////
sudo chown -R $(whoami) session_based_authentication/

### The script needs to be marked as executable and then launched:
chmod +x ./scripts/init_redis.sh
./script/init_redis.sh

### see logs with cargo test
cargo test -- --nocapture

### run integration tests
cargo test integration
(integration tests will fail if they run with unit tests)

### run unit tests
cargo test unit

### run continious integration tests
cargo test ci