# Rust Tutorials

## Rust installation

- [Windows](https://www.rust-lang.org/tools/install)
- [Ubuntu](https://www.journaldev.com/39960/install-rust-on-ubuntu-linux)

### Ubuntu usage quick reference

```bash
curl https://sh.rustup.rs -sSf | sh  # installation
source $HOME/.cargo/env  # configure current shell
rustc --version  # check rust version
cargo --version  # check cargo (Rust's package manager) version
cargo build  # build the project
rustup update  # update rust
```

#### setup directory

```bash
mkdir ~/rustprojects  # make a rust directory
cd ~/rustprojects
mkdir testdir  # make a dir for test code
```

#### add some code

```bash
cd testdir
vi main.rs  # create test app
```

> ```rust
> fn main() {
>     println!("Hello, World!");
> }
> ```

#### compile & run the code

```bash
rustc main.rs  # compile test app
./main  # run test app
```

### Alternative Rust build & Run using Cargo

```bash
cargo new hello-world  # create and setup and new project (like 'go mod init')
cargo run  # build and run the code
```

## Uninstall Rust

```bash
rustup self uninstall  # uninstall rust
```

NOTE: be sure to run `cargo build` after adding dependencies in the *Cargo.toml* file

---


