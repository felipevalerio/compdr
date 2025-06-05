mod tokentype;

struct Token {

	type_: TokenType,
	lexeme: String,
	literal: Option<Box<dyn std::fmt::Debug>>,
	line: u32
}

impl Token {

	pub fn Token(type_: TokenType, lexeme: String, literal: Option<Box<dyn std::fmt::Debug>>, line: u32) -> Self {
		Token{
			type_,
			lexeme,
			literal,
			line,
		}
	}

	pub fn Display(&self) -> String {
		format!("{} {} {}", self.type_, self.lexeme, self.literal)
	}
}