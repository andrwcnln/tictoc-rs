# Time any command.

![badge](https://github.com/andrwcnln/tictoc/actions/workflows/rust.yml/badge.svg)

## Installation
### Option 1: Download the relevant binary for your system from the [Releases](https://github.com/andrwcnln/tictoc/releases) page.
Simply download the binary and place it in the directory of your choice. You may have to add the files to the path for them to be correctly called.

### Option 2: Build from source.
Firstly, make sure you have [cargo](https://github.com/rust-lang/cargo) installed on your system.

Then, clone this repository.
```
git clone https://github.com/andrwcnln/tictoc
```

From the project folder, run:
```
cargo build -r
```
(the `-r` flag specifies the release build. Learn more [here](https://doc.rust-lang.org/cargo/commands/cargo-build.html).)

The binary will be built and located at `PROJECT_DIR/target/release`.

## Usage
Simply prepend any command you wish to time with `tictoc`. It should work with any command and any arguments.
```
$ tictoc sleep 5
time elapsed 5.002633615
```
