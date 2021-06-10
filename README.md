# tufa_backend
Status of last deployment: <br/>
<img src="https://github.com/kuqmua/tufa_backend/workflows/CI/badge.svg?branch=master"><br/>

## change project config
./config <br/>

## change project constants
.libs/config_lib/src/get_project_information/project_constans.rs <br/>

## change tests constants
.libs/config_lib/src/get_project_information/project_constans.rs <br/>

## start docker daemon
sudo dockerd

## start command
cd libs/tests_lib && cargo test local && cd .. && cd .. && cargo run

## how to install rust in wsl2
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential
source $HOME/.cargo/env
sudo apt install pkg-config //optionally
sudo apt-get install pkg-config libssl-dev
rustup default nightly
rustup update //update every week cause nightly

## how to install rust in windows 10
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