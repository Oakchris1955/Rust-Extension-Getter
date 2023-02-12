# Rust Extension Getter

A simple program written in Rust that extracts the extension from a filename

## Installation

1) Install Rust acccording to <https://www.rust-lang.org/tools/install>
2) Clone this repository either using Git or by downloading it using the green button saying `Code` and then extracting it
3) Open a shell and `cd` to the direcotry containing the cloned repository
4) Type `cargo build --release` to build a release build. It will be located in `target/release` (You can also run it directly by typing `cargo run --release`)

## Usage

The program takes one argument; the filename. Then, the extension is extracted from the filename and outputted in the console (Note: the program won't check if the filename given exists; all it does is extract the filename extension).
