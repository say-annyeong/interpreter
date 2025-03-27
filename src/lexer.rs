use crate::token::{ASSIGN, COMMA, EOF, FUNCTION, IDENT, ILLEGAL, INT, LBRACE, LET, LPAREN, PLUS, RBRACE, RPAREN, SEMICOLON, Token, lookup_ident};

#[derive(Default, Debug)]
struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char
}

impl Lexer {
    fn new(input: String) -> Self {
        let mut lexer = Lexer { input, ..Default::default() };
        lexer.read_char(); // read EOF
        lexer
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '=' => Token::new(ASSIGN.to_string(), self.ch.to_string()),
            ';' => Token::new(SEMICOLON.to_string(), self.ch.to_string()),
            '(' => Token::new(LPAREN.to_string(), self.ch.to_string()),
            ')' => Token::new(RPAREN.to_string(), self.ch.to_string()),
            ',' => Token::new(COMMA.to_string(), self.ch.to_string()),
            '+' => Token::new(PLUS.to_string(), self.ch.to_string()),
            '{' => Token::new(LBRACE.to_string(), self.ch.to_string()),
            '}' => Token::new(RBRACE.to_string(), self.ch.to_string()),
            '\0' => Token::new(EOF.to_string(), '\0'.to_string()),
            _ => if is_letter(self.ch) {
                println!("letter trigger: {}", self.ch);
                let literal = self.read_identifier();
                println!("identifier: {}", literal);
                Token::new(lookup_ident(&literal).to_string(), literal)
            } else if is_digit(self.ch) {
                let number = self.read_number();
                println!("number: {}", number);
                Token::new(INT.to_string(), number)
            } else {
                println!("Illegal character: {}", self.ch);
                Token::new(ILLEGAL.to_string(), self.ch.to_string())
            }
        };
        println!("next_token:: ch: {}, pos: {}, read_pos: {}", self.ch, self.position, self.read_position);
        self.read_char();

        // println!("주변 글자: {}", self.input.get(self.position.saturating_sub(1)..=self.position+1).unwrap());
        tok
    }

    fn read_char(&mut self) {
        if self.position >= self.input.len() {
            self.ch = Default::default();
        } else {
            let Some(ch) = self.input.chars().nth(self.position) else {
                panic!("{}번째 문자가 없음!", self.position);
            };
            self.ch = ch;
        }
        self.read_position += 1;
        eprintln!("NEXT:: Char: {}, cur_position: {}, peek_position: {}", self.ch, self.position, self.read_position);
        self.position = self.read_position;


    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            Default::default()
        } else {
            eprintln!("PEEK:: Char: {}, cur_position: {}, peek_position: {}", self.ch, self.position, self.read_position);
            self.input.chars().nth(self.position).unwrap()
        }

    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        println!("read_identifier start:: ch: {}, pos: {}", self.ch, self.position);
        while is_letter(self.peek_char()) {
            self.read_char();
        }
        println!("read_identifier exit:: ch: {}, pos: {}", self.ch, self.position);

        let literal = self.input.get(position-1..self.position).unwrap().to_string();
        literal
    }

    fn read_number(&mut self) -> String {
        let positions = self.position;
        println!("read_number start:: ch: {}, pos: {}", self.ch, self.position);
        while is_digit(self.peek_char()) {
            self.read_char();
        }
        println!("read_number exit:: ch: {}, pos: {}", self.ch, self.position);

        let number = self.input.get(positions-1..self.position).unwrap().to_string();
        number
    }
}

pub fn test_next_token(t: String) {
    let input = "let five = 5;\nlet ten = 10;\nlet add = fn(x, y) {\nx + y;\n};\nlet result = add(five, ten);\n".to_string();

    let tests = vec![
        Token::new(LET.to_string(), "let".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(IDENT.to_string(), "five".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(ASSIGN.to_string(), "=".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(INT.to_string(), "5".to_string()),
        Token::new(SEMICOLON.to_string(), ";".to_string()),
        Token::new(ILLEGAL.to_string(),"\n".to_string()),
        Token::new(LET.to_string(), "let".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(IDENT.to_string(), "ten".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(ASSIGN.to_string(), "=".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(INT.to_string(), "10".to_string()),
        Token::new(SEMICOLON.to_string(), ";".to_string()),
        Token::new(ILLEGAL.to_string(),"\n".to_string()),
        Token::new(LET.to_string(), "let".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(IDENT.to_string(), "add".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(ASSIGN.to_string(), "=".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(FUNCTION.to_string(), "fn".to_string()),
        Token::new(LPAREN.to_string(), "(".to_string()),
        Token::new(IDENT.to_string(), "x".to_string()),
        Token::new(COMMA.to_string(), ",".to_string()),
        Token::new(ILLEGAL.to_string(), " ".to_string()),
        Token::new(IDENT.to_string(), "y".to_string()),
        Token::new(RPAREN.to_string(), ")".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(LBRACE.to_string(), "{".to_string()),
        Token::new(ILLEGAL.to_string(),"\n".to_string()),
        Token::new(IDENT.to_string(), "x".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(PLUS.to_string(), "+".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(IDENT.to_string(), "y".to_string()),
        Token::new(SEMICOLON.to_string(), ";".to_string()),
        Token::new(ILLEGAL.to_string(),"\n".to_string()),
        Token::new(RBRACE.to_string(), "}".to_string()),
        Token::new(SEMICOLON.to_string(), ";".to_string()),
        Token::new(ILLEGAL.to_string(),"\n".to_string()),
        Token::new(LET.to_string(), "let".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(IDENT.to_string(), "result".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(ASSIGN.to_string(), "=".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(IDENT.to_string(), "add".to_string()),
        Token::new(LPAREN.to_string(), "(".to_string()),
        Token::new(IDENT.to_string(), "five".to_string()),
        Token::new(COMMA.to_string(), ",".to_string()),
        Token::new(ILLEGAL.to_string()," ".to_string()),
        Token::new(IDENT.to_string(), "ten".to_string()),
        Token::new(RPAREN.to_string(), ")".to_string()),
        Token::new(SEMICOLON.to_string(), ";".to_string()),
        Token::new(ILLEGAL.to_string(),"\n".to_string()),
        Token::new(EOF.to_string(), '\0'.to_string()),
    ];

    let mut l = Lexer::new(input);
    let mut res = Vec::new();
    for (i, tt) in tests.iter().enumerate() {
        let tok = l.next_token();
        println!("tok: {:?}", tok);
        if tok.token_type != tt.token_type {
            panic!("tests[{}] - token type wrong. expected={:?}, got={:?}", i, tt.token_type, tok.token_type);
        }
        if tok.literal != tt.literal {
            panic!("tests[{}] - literal wrong. expected={:?}, got={:?}", i, tt.literal, tok.literal);
        }
        res.push(tok);
    }

    println!("ok: {:?}", res);
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}