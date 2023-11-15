use std::{
	fmt::Display,
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

	fn colorize<T>(&self, s: T) -> String
	where T: Display + OwoColorize {
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

#[derive(Parser)]
struct Args {
	src: PathBuf,
}

fn main() {
	let args = Args::parse();

	let src = read_to_string(args.src).unwrap();

	let stdparser = StreamParser(stdin().lines().map(|l| l.unwrap()));

	let mut o = stdout().lock();
	let mut last = 0;

	for span in stdparser {
		let Ok(ColoredSpan { span, color }) = span else {
			println!("BOOOOOMM!@#@#");
			panic!()
		};

		// println!("{:?}", span.color(color));

		write!(o, "{}", &src[last..span.start]).unwrap();
		last = span.start;

		write!(
			o,
			"{}",
			color.colorize(&src[last..span.end])
		)
		.unwrap();

		last = span.end;
	}
}
