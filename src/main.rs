use std::{
	fs::read_to_string,
	io::{stdin, stdout, Write},
	ops::Range,
	path::PathBuf,
};

use clap::Parser;
use owo_colors::{DynColors, OwoColorize};

struct FbColor {
	fg: Option<DynColors>,
	bg: Option<DynColors>,
}

impl FbColor {
	fn parse_dyncolor(s: &str) -> Result<DynColors, ()> {
		// currently support only hex colors
		if !matches!(s.chars().next(), Some('#')) {
			Err(())?
		}

		let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| ())?;
		let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| ())?;
		let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| ())?;

		Ok(DynColors::Rgb(r, g, b))
	}

	fn line_paint(&self, s: &str) -> String {
		s.split('\n')
			.map(|s| self.colorize(s))
			.collect::<Vec<_>>()
			.join("\n")
	}

	fn colorize(&self, s: &str) -> String {
		match (self.fg, self.bg) {
			(None, None) => s.to_string(),
			(Some(fg), None) => s.color(fg).to_string(),
			(None, Some(bg)) => s.on_color(bg).to_string(),
			(Some(fg), Some(bg)) => s.color(fg).on_color(bg).to_string(),
		}
	}
}

struct ColoredSpan {
	pub span: Range<usize>,
	pub color: FbColor,
}

struct StreamParser<T>(T)
where T: Iterator<Item = String>;

impl<T> Iterator for StreamParser<T>
where T: Iterator<Item = String>
{
	type Item = Result<ColoredSpan, ()>;

	fn next(&mut self) -> Option<Self::Item> {
		let Some(line) = self.0.next() else {
			return None;
		};

		let mut cols = line
			.splitn(3, [' ', '\t'])
			.filter(|s| !s.is_empty());

		let Some(Ok(s1)) = cols.next().map(|s| s.parse::<usize>()) else {
			return Some(Err(()));
		};

		let Some(Ok(s2)) = cols.next().map(str::parse::<usize>) else {
			return Some(Err(()));
		};

		// should have at least one color
		let c1 = cols.next().ok_or(()).unwrap();

		// NOTE: guarantedd to have at least one character due to split-filter
		let isbg1 = c1.chars().next().unwrap() == '!';

		// remove `!` if it is bg marker
		let c1 = Some(
			FbColor::parse_dyncolor(if isbg1 { &c1[1..] } else { c1 }).unwrap(),
		);

		// second color is optional
		let c2 = if let Some(c2) = cols.next() {
			// NOTE: guarantedd to have at least one character due to split-filter
			let isbg2 = c2.chars().next().unwrap() == '!';

			// if one is fg, then the other should be bg.
			if isbg1 == isbg2 {
				return Some(Err(()));
			}

			// remove `!` if it is bg marker
			Some(
				FbColor::parse_dyncolor(if isbg2 { &c2[1..] } else { c2 })
					.unwrap(),
			)
		} else {
			None
		};

		Some(Ok(ColoredSpan {
			span: s1..s2,
			color: if isbg1 {
				FbColor { fg: c2, bg: c1 }
			} else {
				FbColor { fg: c1, bg: c2 }
			},
		}))
	}
}

struct Highlighter<'a, I, O>
where
	I: Iterator<Item = String>,
	O: Write, {
	source: &'a str,
	input: StreamParser<I>,
	output: O,
}

impl<'a, I, O> Highlighter<'a, I, O>
where
	I: Iterator<Item = String>,
	O: Write,
{
	pub fn new(source: &'a str, input: I, output: O) -> Self {
		Self {
			source,
			input: StreamParser(input),
			output,
		}
	}

	pub fn highlight(mut self) -> Result<(), ()> {
		let mut cursor = 0;

		for span in self.input {
			let Ok(ColoredSpan { span, color }) = span else {
				Err(())?
			};

			self.output
				.write(&self.source[cursor..span.start].as_bytes())
				.map_err(|_| ())?;

			cursor = span.end;

			self.output
				.write(
					color
						.line_paint(&self.source[span])
						.as_bytes(),
				)
				.map_err(|_| ())?;
		}

		self.output
			.write(&self.source[cursor..].as_bytes())
			.unwrap();

		Ok(())
	}
}

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
