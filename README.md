# tufa_backend
Status of last deployment: <br/>
<img src="https://github.com/kuqmua/tufa_backend/workflows/CI/badge.svg?branch=master"><br/>

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

### add user in mondodb
sudo docker exec -it MONGO_CONTAINER_ID bash
inside container: 
mongo
inside mongo cli:
use admin
db.createUser(
  {
    user: "your_username",
    pwd: "your_password",
    roles: [ { role: "userAdminAnyDatabase", db: "admin" } ]
  }
)
then exit mongo cli and container

### How to connect Robo 3T (Robomongo) to MongoDB Atlas (cloud mongoDB database)
https://www.youtube.com/watch?v=t_X7qFMmWhI

### start mongodb docker container
with docker: sudo docker start mongo-tufa-wsl2 <br/>
with docker-compose(other services too): sudo docker-compose -f docker-compose.yml up -d 

### install diesel cli for postgres(diesel dependency)
sudo apt install libpq-dev <br/>
cargo install diesel_cli --no-default-features --features postgres

### diesel examples (libs/diesel_demo/)
diesel setup <br/>
diesel migration run <br/>
diesel migration redo <br/>
cargo run --bin write_post <br/>
cargo run --bin publish_post 1 <br/>
cargo run --bin show_posts <br/>
cargo run --bin delete_post (post name)

### run project with sea_orm logs
RUST_LOG=debug cargo run 2>&1 | grep sea_orm

### sea_orm generate entities
https://www.sea-ql.org/SeaORM/docs/generate-entity/sea-orm-cli

### create another rust lib
cd libs
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

### install docker-compose on wsl2
sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose <br/>
sudo chmod +x /usr/local/bin/docker-compose

### how to install rust in wsl2
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh <br/>
sudo apt install build-essential <br/>
source $HOME/.cargo/env <br/>
sudo apt install pkg-config //optionally <br/>
sudo apt-get install pkg-config libssl-dev <br/>
rustup default nightly <br/>
rustup update //update every week cause nightly <br/>
source $HOME/.cargo/env <br/>
source ~/.profile <br/>

### how to add ssh key on wsl2
cd ~/.ssh
ls
ssh-keygen -o
cat ~/.ssh/id_rsa.pub

### how to install docker on wsl2
sudo apt-get update
sudo apt-get install \
    apt-transport-https \
    ca-certificates \
    curl \
    gnupg \
    lsb-release
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io


### how to install wsl in windows 11
open cmd with administator rights
wsl --install
(default will be ubuntu)

### how to install rust in windows 10
https://estada.ch/2020/4/19/installing-rust-on-windows-10-native/ 

This is a quick how to for Windows 10 2004.

Download and run rustup.rs
Download Build Tools for Visual Studio 2019 hidden under "Tools for Visual Studio 2019"
Run "Build Tools for Visual Studio 2019 Installer" and select
C++ Tools
While in the C++ Tools you have to select "Windows 10 SDK" as well. There will be multiple options, go with the highest version number.
Testing the installation
Open a new PowerShell or "Command Prompt" and type the following commands. Each one must return without error.
Go to the temporary directory: cd %TEMP%
Create a new test project: cargo new toolchain_test
Enter the newly created directory: cd toolchain_test
Build and run the "Hello, world!" program: cargo run
Now you should see some compiling and then Hello, world!

Possible errors
The tools/compiler just stopped working
Under Windows you have to keep track of which environment you are using.

Command Prompt / cmd and PowerShell are the native Windows environment. It is the same as when you start a program from the start menu.
Windows Subsystem for Linux (WSL) or WSL 2 are mostly isolated POSIX environments running on top of Windows. Applications installed inside a WSL might not be visible from outside.
MINGW64 is similar to WSL 1. It is an overlay installation designed to run Linux programs on Windows. It is sometimes installed my other software like git.
You can use rustup inside the encapsulated environments too. However, then the programs created depend on their environment. So you would have to cross-compile them for stable-x86_64-pc-windows-msvc the native Windows environment.

cl.exe exits with error code 2 or "stdlib.h" not found:
You are probably missing the "Windows 10 SDK".

Restart the "Build Tools for Visual Studio 2019 Installer"
In the tile "Visual Studio Build Tools 2019" press "Modify"
Under C++ Tools enable the "Windows 10 SDK", which at the time of writing is 10.0.18362.0.
(picture)
cargo command not found
You are probably missing cargo in your %PATH%.

First, Open a new and fresh shell and try there. The %PATH% variable is only loaded when the terminal starts.
If that did not work, you can execute rustup.exe again and re-install the rust toolchain
Or you can edit the PATH variable by hand in the computers settings.
