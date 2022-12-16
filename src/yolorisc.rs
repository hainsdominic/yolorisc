use crate::alu::{Alu, OpAlu};

pub struct YoloRisc {
    pub registers: [u8; 8],
    pub program_counter: usize,
    alu: Alu,
}

impl YoloRisc {
    pub fn new() -> YoloRisc {
        YoloRisc {
            registers: [0; 8],
            program_counter: 0,
            alu: Alu::new(),
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

pub struct Clock {
    cycles: u64,
}

impl Clock {
    pub fn new() -> Clock {
        Clock { cycles: 0 }
    }

    pub fn tick(&mut self) {
        self.cycles += 1;
    }
}
