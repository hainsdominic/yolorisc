use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::yolorisc::{Instruction, OpCodes};

pub fn instructions_from_file(filename: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().filter(|line| {
        !line.as_ref().unwrap().starts_with("--") && !line.as_ref().unwrap().is_empty()
    }) {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();

        let opcode = usize::from_str_radix(parts.next().unwrap(), 2)
            .unwrap_or_else(|_| panic!("Invalid instruction: {}", line));
        let dst = usize::from_str_radix(parts.next().unwrap(), 2)
            .unwrap_or_else(|_| panic!("Invalid instruction: {}", line));
        let lhs = usize::from_str_radix(parts.next().unwrap(), 2)
            .unwrap_or_else(|_| panic!("Invalid instruction: {}", line));
        let rhs = usize::from_str_radix(parts.next().unwrap(), 2)
            .unwrap_or_else(|_| panic!("Invalid instruction: {}", line));

        let opcode = OpCodes::try_from(opcode as i32)
            .unwrap_or_else(|_| panic!("Invalid instruction: {}", line));

        instructions.push(Instruction {
            opcode,
            dst,
            lhs,
            rhs,
        });
    }

    instructions
}
