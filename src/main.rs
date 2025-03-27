mod lexer;
mod token;

fn main() {
    let a = String::new();
    lexer::test_next_token(a);
}