use crate::alu::{Alu, OpAlu};

pub struct YoloRisc {
    pub registers: [u8; 8],
    pub program_counter: usize,
    pub memory: [u8; 256],
    alu: Alu,
}

impl Default for YoloRisc {
    fn default() -> Self {
        Self::new()
    }
}

impl YoloRisc {
    pub fn new() -> YoloRisc {
        YoloRisc {
            registers: [0; 8],
            program_counter: 0,
            memory: [0; 256],
            alu: Alu::default(),
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match &instruction.opcode {
            OpCodes::ALU(opcode) => {
                let result = self.alu.execute(
                    opcode,
                    self.registers[instruction.lhs],
                    self.registers[instruction.rhs],
                );
                self.registers[instruction.dst] = result;
            }
            OpCodes::MOV => {
                self.registers[instruction.dst] = instruction.lhs as u8;
            }
            OpCodes::PRT => {
                println!(
                    "Register {} = {}",
                    instruction.dst, self.registers[instruction.dst]
                );
            }
            OpCodes::END => {
                println!("Program ended");
                std::process::exit(0);
            }
            OpCodes::ST => {
                self.memory[instruction.dst] = self.registers[instruction.lhs];
            }
            OpCodes::LD => {
                self.registers[instruction.dst] = self.memory[instruction.lhs];
            }
            OpCodes::JZ => {
                if self.alu.zero {
                    self.program_counter = instruction.lhs - 1;
                }
            }
            OpCodes::JNZ => {
                if !self.alu.zero {
                    self.program_counter = instruction.lhs - 1;
                }
            }
            OpCodes::JMP => {
                self.program_counter = instruction.lhs - 1;
            }
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum OpCodes {
    ALU(OpAlu),
    MOV = 6,
    PRT = 7,
    END = 8,
    ST = 9,
    LD = 10,
    JZ = 11,
    JNZ = 12,
    JMP = 13,
}

impl TryFrom<i32> for OpCodes {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(OpCodes::ALU(OpAlu::ADD)),
            1 => Ok(OpCodes::ALU(OpAlu::SUB)),
            2 => Ok(OpCodes::ALU(OpAlu::AND)),
            3 => Ok(OpCodes::ALU(OpAlu::OR)),
            4 => Ok(OpCodes::ALU(OpAlu::XOR)),
            5 => Ok(OpCodes::ALU(OpAlu::NOT)),
            6 => Ok(OpCodes::MOV),
            7 => Ok(OpCodes::PRT),
            8 => Ok(OpCodes::END),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: OpCodes,
    pub dst: usize,
    pub lhs: usize,
    pub rhs: usize,
}

#[derive(Default)]
pub struct Clock {
    cycles: u64,
}

impl Clock {
    pub fn tick(&mut self) {
        self.cycles += 1;
    }
}
