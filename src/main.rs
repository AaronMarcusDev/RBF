use std::fs;
use std::env;
mod lexer;

fn load_file(file_path: &str) -> String {
    return fs::read_to_string(file_path)
        .expect("[ERROR] Unable to read file.");
}

fn main() {
    // Command line arguments
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    if args.len() != 1 {
        println!("Usage: rbf <file>");
    } else {
        // Load file
        let content: String = load_file("./main.b");
    
        //Calling lexer
        let _tokens: Vec<lexer::Instruction> = lexer::lex(content);
    }

}