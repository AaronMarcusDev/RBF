#[derive(Debug)]
pub enum Command {
    Increment,
    Decrement,
    ShiftLeft,
    ShiftRight,
    LoopStart,
    LoopEnd,
    CharOut,
    CharIn,
}

pub fn lex(input: String) -> Vec<Command> {
    let mut tokens: Vec<Command> = Vec::new();
    for c in input.split("") {
        match c {
            "+" => tokens.push(Command::Increment),
            "-" => tokens.push(Command::Decrement),
            "<" => tokens.push(Command::ShiftLeft),
            ">" => tokens.push(Command::ShiftRight),
            "[" => tokens.push(Command::LoopStart),
            "]" => tokens.push(Command::LoopEnd),
            "." => tokens.push(Command::CharOut),
            "," => tokens.push(Command::CharIn),
            _ => (),
        }
    }
    return tokens;
} 
