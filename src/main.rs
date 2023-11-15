use std::{io::stdin, ops::Range};

use owo_colors::{DynColors, OwoColorize};

struct ColoredSpan {
	pub span: Range<usize>,
	pub color: DynColors,
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

		let c = cols.next().unwrap();

		let r = u8::from_str_radix(&c[1..3], 16).unwrap();
		let g = u8::from_str_radix(&c[3..5], 16).unwrap();
		let b = u8::from_str_radix(&c[5..7], 16).unwrap();

		Some(Ok(ColoredSpan {
			span: s1..s2,
			color: DynColors::Rgb(r, g, b),
		}))
	}
}

fn main() {
	let stdparser = StreamParser(stdin().lines().map(|l| l.unwrap()));

	for span in stdparser {
		let Ok(ColoredSpan { span, color }) = span else {
			println!("BOOOOOMM!@#@#");
			panic!()
		};
		println!("{:?}", span.color(color));
	}
}
