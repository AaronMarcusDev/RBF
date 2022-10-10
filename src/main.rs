mod interpreter;
mod lexer;

fn load_file(file_path: &str) -> String {
    if std::path::Path::new(file_path).exists() {
        return std::fs::read_to_string(file_path).expect("[ERROR] Unable to read file.");
    } else {
        println!("[ERROR] File does not exist.");
        std::process::exit(1);
    }
}

fn main() {
    // Command line arguments
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    if args.len() != 1 {
        println!("[ERROR] Invalid number of arguments.");
        println!("[INFO] Usage: rbf <file>");
    } else {
        // Load file
        let content: String = load_file(args[0].as_str());

        if content.trim().len() != 0 {
            //Calling lexer
            let tokens: Vec<lexer::Instruction> = lexer::lex(content);
            interpreter::interpret(tokens);
        }
    }
}
