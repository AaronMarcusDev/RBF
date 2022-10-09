use std::fs;
mod lexer;

fn load_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("[ERROR] Unable to read file.");
}

fn main() {
    let content: String = load_file("./main.b");
    let tokens: Vec<lexer::Command> = lexer::lex(content);
    println!("{:?}", tokens);
}