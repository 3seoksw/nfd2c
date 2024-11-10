# NFD2C (NFD to NFC)

The following program resolves the incompatibility issue of file name written with Korean between macOS and Windows, etc..
맥OS에서 파일 이름이 한글일 경우, 자소분리가 되어 Windows와 같은 OS에서 한글이 깨지는 문제를 해결하는 프로그램.

## Installation (설치)

### Prerequisites

- Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed. You can install it by running:

```zsh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Run the following to see whether the prerequisites are installed properly:

```zsh
cargo --version
```

### Building

```zsh
git clone git@github.com:3seoksw/nfd2c.git
```

Download the latest release by running the above command or from [GitHub Releases](https://github.com/3seoksw/nfd2c/releases).

Navigate into the project directory by `cd` command and build the project with `cargo`.

```zsh
cargo build --release
sudo cp ./target/release/nfd2c /usr/local/bin
```

## Run

```zsh
nfd2c -d path/to/files/for/directory
```
