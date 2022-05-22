# Rust Tutorials

## Rust installation

- [Windows](https://www.rust-lang.org/tools/install)
- [Ubuntu](https://www.journaldev.com/39960/install-rust-on-ubuntu-linux)

### Ubuntu usage quick reference

```bash
curl https://sh.rustup.rs -sSf | sh  # installation
source $HOME/.cargo/env  # configure current shell
rustc --version  # check rust version
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
vi rusttestfile.rs  # create test app
```

> ```rust
> fn main() {
>     println!("Hello, World!");
> }
> ```

#### compile & run the code

```bash
rustc rusttestfile.rs  # compile test app
./rusttestfile  # run test app
```

## Uninstall Rust

```bash
rustup self uninstall  # uninstall rust
```

---

