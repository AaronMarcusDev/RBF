use crate::lexer;

pub fn interpret(tokens: Vec<lexer::Instruction>) {
    let mut mem: Vec<u8> = vec![0; 30000];
    let mut ptr: u32 = 0;

    for command in tokens {
        match command {
            lexer::Instruction::ShiftLeft => {
                if ptr == 0 {
                    ptr = mem.len() as u32;
                } else {
                    ptr -= 1;
                }
            }
            lexer::Instruction::ShiftRight => {
                if ptr == mem.len() as u32 {
                    ptr = 0;
                } else {
                    ptr += 1;
                }
            }
            lexer::Instruction::Increment => mem[ptr as usize] += 1,
            lexer::Instruction::Decrement => {
                if mem[ptr as usize] != 0 {
                    mem[ptr as usize] -= 1
                }
            }
            lexer::Instruction::CharIn => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("[ERROR] Unable to read input.");

                // convert string to bytes
                mem[ptr as usize] = input.as_bytes()[0] as u8;
            }
            lexer::Instruction::CharOut => print!("{}", mem[ptr as usize] as char),
            lexer::Instruction::LoopStart => (),
            lexer::Instruction::LoopEnd => (),
        }
    }
}
