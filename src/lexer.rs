use logos::Logos;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Logos)]
#[logos(skip r#"[ \t\n\f\r]+"#)]
pub enum Token<'a> {
	#[token(r#"&"#)]
	And,
	#[token(r#"&&"#)]
	AndAnd,
	#[token(r#"|"#)]
	Or,
	#[token(r#"||"#)]
	OrOr,
	#[token(r#"("#)]
	LeftParen,
	#[token(r#")"#)]
	RightParen,
	#[token(r#"{"#)]
	LeftBrace,
	#[token(r#"}"#)]
	RightBrace,
	#[token(r#"!"#)]
	Bang,
	#[token(r#";"#)]
	Semicolon,
	#[token(r#":"#)]
	Colon,
	#[token(r#"."#)]
	Dot,
	#[token(r#","#)]
	Comma,
	#[token(r#"let"#)]
	Let,
	#[token(r#"="#)]
	Eq,
	#[token(r#"=="#)]
	EqEq,
	#[token(r#"loop"#)]
	Loop,
	#[token(r#"while"#)]
	While,
	#[token(r#"continue"#)]
	Continue,
	#[token(r#"struct"#)]
	Struct,
	#[token(r#"enum"#)]
	Enum,
	#[token(r#"fn"#)]
	Fn,
	#[token(r#"break"#)]
	Break,
	#[token(r#"if"#)]
	If,
	#[token(r#"else"#)]
	Else,
	#[regex(r#""([^"\\]|\\[\s\S])*""#, |lex| {
        let slice = lex.slice();
        &slice[1..slice.len() - 1]
    })]
	String(&'a str),
	#[regex(r#"[a-zA-Z$_][a-zA-Z0-9$_]*"#)]
	Ident(&'a str),
	#[regex(r#"-\d*|\d*"#)]
	Int(&'a str),
	#[regex(r#"\d*\.\d+"#)]
	Float(&'a str),
	#[token(r#"return"#)]
	Return,
	#[token(r#"true"#)]
	True,
	#[token(r#"false"#)]
	False,
	#[token(r#"->"#)]
	RArrow,
	#[token(r#"=>"#)]
	FatArrow,
}
