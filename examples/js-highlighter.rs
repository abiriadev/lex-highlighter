use std::{fs::read_to_string, path::PathBuf};

use clap::Parser;
use swc_common::{BytePos, Span};
use swc_ecma_parser::{
	lexer::Lexer,
	token::{
		BinOpToken, IdentLike, Keyword, KnownIdent, Token, TokenAndSpan, Word,
	},
	StringInput, Syntax, TsConfig,
};

fn token_to_color(token: Token) -> u32 {
	match token {
		Token::Word(w) => match w {
			Word::Keyword(k) => match k {
				Keyword::Await
				| Keyword::Break
				| Keyword::Case
				| Keyword::Catch
				| Keyword::Continue
				| Keyword::Debugger
				| Keyword::Default_
				| Keyword::Do
				| Keyword::Else
				| Keyword::Finally
				| Keyword::For
				| Keyword::Function
				| Keyword::If
				| Keyword::Return
				| Keyword::Switch
				| Keyword::Throw
				| Keyword::Try
				| Keyword::Var
				| Keyword::Let
				| Keyword::Const
				| Keyword::While
				| Keyword::With
				| Keyword::New
				| Keyword::This
				| Keyword::Super
				| Keyword::Class
				| Keyword::Extends
				| Keyword::Export
				| Keyword::Import
				| Keyword::Yield
				| Keyword::In
				| Keyword::InstanceOf
				| Keyword::TypeOf
				| Keyword::Void
				| Keyword::Delete => 0xC678DD,
			},
			Word::Null | Word::True | Word::False => 0xD19A66,
			Word::Ident(i) => match i {
				IdentLike::Known(k) => match k {
					KnownIdent::Abstract
					| KnownIdent::As
					| KnownIdent::Async
					| KnownIdent::From
					| KnownIdent::Of
					| KnownIdent::Type
					| KnownIdent::Global
					| KnownIdent::Static
					| KnownIdent::Using
					| KnownIdent::Readonly
					| KnownIdent::Unique
					| KnownIdent::Keyof
					| KnownIdent::Declare
					| KnownIdent::Enum
					| KnownIdent::Is
					| KnownIdent::Infer
					| KnownIdent::Symbol
					| KnownIdent::Undefined
					| KnownIdent::Interface
					| KnownIdent::Implements
					| KnownIdent::Asserts
					| KnownIdent::Require
					| KnownIdent::Get
					| KnownIdent::Set
					| KnownIdent::Any
					| KnownIdent::Intrinsic
					| KnownIdent::Unknown
					| KnownIdent::String
					| KnownIdent::Object
					| KnownIdent::Number
					| KnownIdent::Bigint
					| KnownIdent::Boolean
					| KnownIdent::Never
					| KnownIdent::Assert
					| KnownIdent::Namespace
					| KnownIdent::Accessor
					| KnownIdent::Meta
					| KnownIdent::Target
					| KnownIdent::Satisfies
					| KnownIdent::Package
					| KnownIdent::Protected
					| KnownIdent::Private
					| KnownIdent::Public => 0x61AFEF,
					_ => unimplemented!(),
				},
				IdentLike::Other(_) => 0xE06C75,
			},
		},
		Token::Arrow
		| Token::Hash
		| Token::At
		| Token::Dot
		| Token::DotDotDot
		| Token::Bang => 0x56B6C2,
		Token::LParen | Token::RParen => 0xC678DD,
		Token::LBracket | Token::RBracket => 0x56B6C2,
		Token::LBrace | Token::RBrace => 0x3FD7A9, // #56B6C2 b #D19A66 y #C678DD p
		Token::Semi => 0x214365,
		Token::Comma => 0xABB2BF,
		Token::BackQuote => 0x98C379,
		Token::Template { .. } => 0x98C379,
		Token::Colon => 0xABB2BF,
		Token::BinOp(b) => match b {
			BinOpToken::EqEq
			| BinOpToken::NotEq
			| BinOpToken::EqEqEq
			| BinOpToken::NotEqEq
			| BinOpToken::Lt
			| BinOpToken::LtEq
			| BinOpToken::Gt
			| BinOpToken::GtEq
			| BinOpToken::LShift
			| BinOpToken::RShift
			| BinOpToken::ZeroFillRShift
			| BinOpToken::Add
			| BinOpToken::Sub
			| BinOpToken::Mul
			| BinOpToken::Div
			| BinOpToken::Mod
			| BinOpToken::BitOr
			| BinOpToken::BitXor
			| BinOpToken::BitAnd
			| BinOpToken::Exp
			| BinOpToken::LogicalOr
			| BinOpToken::LogicalAnd
			| BinOpToken::NullishCoalescing => 0x56B6C2,
		},
		Token::AssignOp(_) => 0x56B6C2,
		Token::DollarLBrace => 0x3FD7BD,
		Token::QuestionMark
		| Token::PlusPlus
		| Token::MinusMinus
		| Token::Tilde => 0xABB2BF,
		Token::Str { .. } => 0x98C379,
		Token::Regex(_, _) => unimplemented!(),
		Token::Num { .. } => 0xD19A66,
		Token::BigInt { .. } => unimplemented!(),
		Token::JSXName { .. } => 0xD19A66,
		Token::JSXText { .. } => 0xABB2BF,
		Token::JSXTagStart | Token::JSXTagEnd => 0xE06C75,
		Token::Shebang(_) => 0x7F848E,
		Token::Error(_) => 0xFF0000,
	}
}

#[derive(Parser)]
struct Args {
	src: PathBuf,
}

fn main() -> anyhow::Result<()> {
	let args = Args::parse();

	let src = read_to_string(args.src)?;

	let lexer = Lexer::new(
		Syntax::Typescript(TsConfig {
			tsx: true,
			decorators: true,
			..Default::default()
		}),
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
		token,
		..
	} in lexer
	{
		println!(
			"{} {} #{:0>6x}",
			lo.0,
			hi.0,
			token_to_color(token)
		);
	}

	Ok(())
}
