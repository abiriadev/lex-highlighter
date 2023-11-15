use std::{
	fs::read_to_string,
	io::{stdin, stdout},
	path::PathBuf,
};

use clap::Parser;
use lex_highlighter::Highlighter;

#[derive(Parser)]
struct Args {
	src: PathBuf,
}

fn main() {
	let args = Args::parse();

	let src = read_to_string(args.src).unwrap();

	let highligher = Highlighter::new(
		&src,
		stdin().lines().map(|l| l.unwrap()),
		stdout().lock(),
	);

	highligher.highlight().unwrap();
}
