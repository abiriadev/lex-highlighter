use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use swc_common::{BytePos, Span};
use swc_ecma_parser::{lexer::Lexer, token::TokenAndSpan, StringInput};

#[derive(Parser)]
struct Args {
	src: PathBuf,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let src = read_to_string(args.src)?;

	let lexer = Lexer::new(
		Default::default(),
		Default::default(),
		StringInput::new(
			&src,
			BytePos(0),
			BytePos(src.len() as u32),
		),
		None,
	);

	for TokenAndSpan {
		span: Span { lo, hi, .. },
		..
	} in lexer
	{
		println!("{} {} {}", lo.0, hi.0, "#ff0000");
	}

	Ok(())
}
