# split_word

This simple cli program will split words in half, indicating where the middle of the word is. I use this program when handwriting text that I want to be centered.

## Installation
Rust and Cargo must be installed. I recommend using [rustup](https://rustup.rs/).

```
git clone https://github.com/z2oh/split_word
cd split_word
cargo build --release
```

The binary is then located at `./target/release/split_word`.

## Usage

```
$ split_word test "spaces supported" oddworkstoo
te|st
spaces s|upported
oddwo|r|kstoo
```
