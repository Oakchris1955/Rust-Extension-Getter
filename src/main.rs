use clap::{arg, command};
use std::process::exit;

fn exit_with_error(msg: &str) -> ! {
	eprintln!("{}", msg);
	exit(1);
}

fn main() {
    let args = command!()
		.about("A simple shell utility written in Rust that returns the extension of a filename (Note: this program won't check if the filename really exists. It will just grab the extension from it)")
		.arg(arg!([filename] "The filename of which to get the extension").required(true))
		.get_matches();

	let filename: &String = args.get_one::<String>("filename").unwrap_or_else(|| {
		exit_with_error("Couldn't get 'filename' parameter");
	});

	let mut split_iterator = filename.split(".");
	split_iterator.next();

	let extension: &str = split_iterator.last().unwrap_or("");

	print!("{}", extension);
}
