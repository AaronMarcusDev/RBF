use crate::lexer;

pub fn interpret(tokens: Vec<lexer::Instruction>) {
    let mut mem: Vec<u8> = vec![0; 30000];
    let mut ptr: u32 = 0;

    for command in tokens {
        match command {
            lexer::Instruction::ShiftLeft => ptr -= 1,
            lexer::Instruction::ShiftRight => ptr += 1,
            lexer::Instruction::Increment => {
                if mem[ptr as usize] == (mem.len() - 1) as u8 {
                    mem[ptr as usize] = 0;
                } else {
                    mem[ptr as usize] += 1;
                }
            }
            lexer::Instruction::Decrement => {
                if mem[ptr as usize] == 0 {
                    mem[ptr as usize] = (mem.len() - 1) as u8;
                } else {
                    mem[ptr as usize] -= 1;
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
