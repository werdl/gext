# Installation
## using `cargo`
### Prerequisites
- you need to have `cargo` installed (which comes with `rust`)
- to do this, run the following command in your terminal:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- this downloads the latest rust installer, then pipes it to your shell to run it
- follow the instructions in the installer (if you don't know what to do, just press enter)
- after the installer is done, you need to restart your terminal (by closing it and opening a new one, or by running `exec <your shell>`), or by running the following command:
```sh
source $HOME/.cargo/env
```
- this adds the `cargo` command to your path
### Installation
- to install `gext`, run the following command in your terminal:
```sh
cargo install gext
```
- this downloads the latest version of `gext` from `crates.io`, then compiles and installs it
- then, assuming the installation of `rust` modified your path, you can run the game with the following command:
```sh
gext [OPTIONS]
```
- if the installation of `rust` did not modify your path, you can run the game with the following command:
```sh
~/.cargo/bin/gext [OPTIONS]
```
- if you are not using *nix, replace `~/.cargo/bin/gext` with the path to the `gext` binary (which should be in the `bin` directory of the `cargo` installation directory, on windows, this is usually `C:\Users\<your username>\.cargo\bin\gext.exe`) 
- if you are using *nix, you can add the `bin` directory of the `cargo` installation directory to your path by running the following command:
```sh
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
```
## using the binary
- the game is built on linux, so it should work on all linux systems
- to install `gext`, download the latest release from the [releases page](https://github.com/werdl/gext/releases)
- ensure to pick the correct binary for your system
- if your system is not listed, you can build the game from source, as detailed above