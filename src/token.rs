mod tokentype;

struct Token {

	token_type: TokenType,
	lexeme: String,
	literal: Option<Box<dyn std::fmt::Debug>>,
	line: u32
}

impl Token {

	pub fn Token(token_type: TokenType, lexeme: String, literal: Option<Box<dyn std::fmt::Debug>>, line: u32) -> Self {
		Token{
			token_type,
			lexeme,
			literal,
			line,
		}
	}
}