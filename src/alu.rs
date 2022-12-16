#[derive(Default)]
pub struct Alu {
    pub carry: bool,
    pub zero: bool,
    pub negative: bool,
}

#[repr(u8)]
#[derive(Debug)]
pub enum OpAlu {
    ADD = 0,
    SUB = 1,
    AND = 2,
    OR = 3,
    XOR = 4,
    NOT = 5,
}

impl Alu {
    pub fn execute(&mut self, opcode: &OpAlu, lhs: u8, rhs: u8) -> u8 {
        match opcode {
            OpAlu::ADD => {
                let result = lhs.overflowing_add(rhs);
                self.carry = result.1;
                self.zero = result.0 == 0;
                self.negative = result.0 > 127;
                result.0
            }
            OpAlu::SUB => {
                let result = lhs.overflowing_sub(rhs);
                self.carry = result.1;
                self.zero = result.0 == 0;
                self.negative = result.0 > 127;
                result.0
            }
            OpAlu::AND => {
                let result = lhs & rhs;
                self.carry = false;
                self.zero = result == 0;
                self.negative = result > 127;
                result
            }
            OpAlu::OR => {
                let result = lhs | rhs;
                self.carry = false;
                self.zero = result == 0;
                self.negative = result > 127;
                result
            }
            OpAlu::XOR => {
                let result = lhs ^ rhs;
                self.carry = false;
                self.zero = result == 0;
                self.negative = result > 127;
                result
            }
            OpAlu::NOT => {
                let result = !lhs;
                self.carry = false;
                self.zero = result == 0;
                self.negative = result > 127;
                result
            }
        }
    }
}
