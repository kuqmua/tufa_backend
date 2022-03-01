Status of last deployment: <br/>
<img src="https://github.com/kuqmua/tufa_backend/workflows/CI/badge.svg?branch=master"><br/>

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
sudo docker build -t tufa_backend-image .

### run docker container
docker run --env-file .env --name tufa_backend-container -p 8000:8000 --rm -it tufa_backend-image

### stop docker container
sudo docker stop tufa_backend-container

### remove docker container
sudo docker rm tufa_backend-container

### remove all unused right now docker containers and images
sudo docker system prune -f 

### run containers with docker-compose
sudo docker-compose up -d

### stop containers with docker-compose
sudo docker-compose down

### pull and run mongodb docker container
(need to write path to your project directory)
sudo docker run -p 27017:27017 --name mongo-tufa-wsl2 -v ~/projects/tufa_backend/mongodb_volume:/data/db -d mongo:latest

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
