enum TokenType {

	// tokens de caracteres únicos
	LEFT_PAREN,
	RIGHT_PAREN,
	LEFT_BRACE,
	RIGHT_BRACE,
	COMMA,
	DOT,
	MINUS,
	PLUS,
	SEMICOLON,
	SLASH,
	STAR,

	// tokens com um ou mais caracteres
	BANG,
	BANG_EQUAL,
	EQUAL,
	EQUAL_EQUAL,
	GREATER,
	GREATER_EQUAL,
	LESS,
	LESS_EQUAL,

	// tokens literais
	IDENTIFIER,
	STRING,
	NUMBER,

	// tokens palavras chaves
	AND,
	CLASS,
	ELSE,
	FALSE,
	TRUE,
	FUN,
	FOR,
	IF,
	NIL,
	OR,
	PRINT,
	RETURN,
	SUPER,
	THIS,
	VAR,
	WHILE,

	// fim do arquivo (input)
	EOF
}