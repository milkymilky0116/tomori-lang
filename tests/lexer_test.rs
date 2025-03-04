use tomori_lang::{lexer::lexer::Lexer, token::token::TokenType};

#[tokio::test]
async fn test_lexer_single_letter() {
    let input = "=+,;(){}";
    let test_cases = vec![
        (TokenType::ASSIGN, "="),
        (TokenType::PLUS, "+"),
        (TokenType::COMMA, ","),
        (TokenType::SEMICOLON, ";"),
        (TokenType::LPAREN, "("),
        (TokenType::RPAREN, ")"),
        (TokenType::LBRACE, "{"),
        (TokenType::RBRACE, "}"),
    ];
    let mut l = Lexer::new(input.to_string());
    for (token_type, literal) in test_cases {
        let tok = l.next_token();
        assert_eq!(tok.token_type, token_type);
        assert_eq!(tok.literal, literal);
    }
}

#[tokio::test]
async fn test_lexer_word() {
    let input = "let five = 5;";
    let test_cases = vec![
        (TokenType::LET, "let"),
        (TokenType::IDENT("five".to_string()), "five"),
        (TokenType::ASSIGN, "="),
        (TokenType::INT(5), "5"),
        (TokenType::SEMICOLON, ";"),
    ];
    let mut l = Lexer::new(input.to_string());
    for (token_type, literal) in test_cases {
        let tok = l.next_token();
        println!("{}", tok.literal);
        assert_eq!(tok.token_type, token_type);
        assert_eq!(tok.literal, literal);
    }
}

#[tokio::test]
async fn test_lexer_multiple_line() {
    let input = "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
}
let result = add(five, ten);
!-/*5;
5 < 10 > 5;
if (5 < 10) {
return true;
} else {
return false;
}
x == y;
10 != 9;
";
    let test_cases = vec![
        (TokenType::LET, "let"),
        (TokenType::IDENT("five".to_string()), "five"),
        (TokenType::ASSIGN, "="),
        (TokenType::INT(5), "5"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::LET, "let"),
        (TokenType::IDENT("ten".to_string()), "ten"),
        (TokenType::ASSIGN, "="),
        (TokenType::INT(10), "10"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::LET, "let"),
        (TokenType::IDENT("add".to_string()), "add"),
        (TokenType::ASSIGN, "="),
        (TokenType::FUNCTION, "fn"),
        (TokenType::LPAREN, "("),
        (TokenType::IDENT("x".to_string()), "x"),
        (TokenType::COMMA, ","),
        (TokenType::IDENT("y".to_string()), "y"),
        (TokenType::RPAREN, ")"),
        (TokenType::LBRACE, "{"),
        (TokenType::IDENT("x".to_string()), "x"),
        (TokenType::PLUS, "+"),
        (TokenType::IDENT("y".to_string()), "y"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::RBRACE, "}"),
        (TokenType::LET, "let"),
        (TokenType::IDENT("result".to_string()), "result"),
        (TokenType::ASSIGN, "="),
        (TokenType::IDENT("add".to_string()), "add"),
        (TokenType::LPAREN, "("),
        (TokenType::IDENT("five".to_string()), "five"),
        (TokenType::COMMA, ","),
        (TokenType::IDENT("ten".to_string()), "ten"),
        (TokenType::RPAREN, ")"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::BANG, "!"),
        (TokenType::MINUS, "-"),
        (TokenType::SLASH, "/"),
        (TokenType::ASTERISK, "*"),
        (TokenType::INT(5), "5"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::INT(5), "5"),
        (TokenType::LT, "<"),
        (TokenType::INT(10), "10"),
        (TokenType::GT, ">"),
        (TokenType::INT(5), "5"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::IF, "if"),
        (TokenType::LPAREN, "("),
        (TokenType::INT(5), "5"),
        (TokenType::LT, "<"),
        (TokenType::INT(10), "10"),
        (TokenType::RPAREN, ")"),
        (TokenType::LBRACE, "{"),
        (TokenType::RETURN, "return"),
        (TokenType::TRUE, "true"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::RBRACE, "}"),
        (TokenType::ELSE, "else"),
        (TokenType::LBRACE, "{"),
        (TokenType::RETURN, "return"),
        (TokenType::FALSE, "false"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::RBRACE, "}"),
        (TokenType::IDENT("x".to_string()), "x"),
        (TokenType::EQ, "=="),
        (TokenType::IDENT("y".to_string()), "y"),
        (TokenType::SEMICOLON, ";"),
        (TokenType::INT(10), "10"),
        (TokenType::NotEQ, "!="),
        (TokenType::INT(9), "9"),
        (TokenType::SEMICOLON, ";"),
    ];
    let mut l = Lexer::new(input.to_string());
    for (token_type, literal) in test_cases {
        let tok = l.next_token();
        println!("{}", tok.literal);
        assert_eq!(tok.token_type, token_type);
        assert_eq!(tok.literal, literal);
    }
}
