#[derive(Debug)]
pub enum Instruction {
    Increment,
    Decrement,
    ShiftLeft,
    ShiftRight,
    LoopStart,
    LoopEnd,
    CharOut,
    CharIn,
}

pub fn lex(input: String) -> Vec<Instruction> {  
    let mut tokens: Vec<Instruction> = Vec::new();

    for c in input.split("") {
        match c {
            "+" => tokens.push(Instruction::Increment),
            "-" => tokens.push(Instruction::Decrement),
            "<" => tokens.push(Instruction::ShiftLeft),
            ">" => tokens.push(Instruction::ShiftRight),
            "[" => tokens.push(Instruction::LoopStart),
            "]" => tokens.push(Instruction::LoopEnd),
            "." => tokens.push(Instruction::CharOut),
            "," => tokens.push(Instruction::CharIn),
            _ => ()
        }
    }
    tokens
}