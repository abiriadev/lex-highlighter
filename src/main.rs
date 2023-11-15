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

	#[arg(long, short)]
	tab: Option<usize>,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let src = read_to_string(args.src)?;

	let highligher = Highlighter::new(
		&src,
		stdin().lines().map(|l| l.unwrap()),
		stdout().lock(),
		args.tab,
	);

	highligher.highlight()?;

	Ok(())
}
