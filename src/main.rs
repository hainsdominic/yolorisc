use yolorisc::{
    reader::instructions_from_file,
    yolorisc::{Clock, YoloRisc},
};

fn main() {
    let program = instructions_from_file("sum.yolo");

    let mut cpu = YoloRisc::new();
    let mut clock = Clock::default();

    loop {
        let instruction = &program[cpu.program_counter];
        cpu.execute(instruction);
        cpu.program_counter += 1;
        clock.tick();

        if cpu.program_counter == program.len() {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use yolorisc::{
        alu::OpAlu,
        yolorisc::{Instruction, OpCodes},
    };

    use super::*;

    fn run_program(program: Vec<Instruction>) -> Vec<u8> {
        let mut cpu = YoloRisc::new();
        let mut clock = Clock::default();

        loop {
            let instruction = &program[cpu.program_counter];
            cpu.execute(instruction);
            cpu.program_counter += 1;
            clock.tick();

            if cpu.program_counter == program.len() {
                break;
            }
        }

        cpu.registers.to_vec()
    }

    #[test]
    fn test_mov() {
        let program = vec![
            Instruction {
                opcode: OpCodes::MOV,
                dst: 0,
                lhs: 1,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 1,
                lhs: 2,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 2,
                lhs: 3,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 3,
                lhs: 4,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 4,
                lhs: 5,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 5,
                lhs: 6,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 6,
                lhs: 7,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 7,
                lhs: 8,
                rhs: 0,
            },
        ];

        let registers = run_program(program);

        assert_eq!(registers, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_add() {
        let program = vec![
            Instruction {
                opcode: OpCodes::MOV,
                dst: 0,
                lhs: 1,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 1,
                lhs: 2,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::ALU(OpAlu::ADD),
                dst: 2,
                lhs: 0,
                rhs: 1,
            },
        ];

        let registers = run_program(program);

        assert_eq!(registers, [1, 2, 3, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_sub() {
        let program = vec![
            Instruction {
                opcode: OpCodes::MOV,
                dst: 0,
                lhs: 1,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 1,
                lhs: 2,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::ALU(OpAlu::SUB),
                dst: 2,
                lhs: 0,
                rhs: 1,
            },
        ];

        let registers = run_program(program);

        assert_eq!(registers, [1, 2, 255, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_xor() {
        let program = vec![
            Instruction {
                opcode: OpCodes::MOV,
                dst: 0,
                lhs: 1,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::MOV,
                dst: 1,
                lhs: 2,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::ALU(OpAlu::XOR),
                dst: 2,
                lhs: 0,
                rhs: 1,
            },
        ];

        let registers = run_program(program);

        assert_eq!(registers, [1, 2, 3, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_memory() {
        let program = vec![
            Instruction {
                opcode: OpCodes::MOV,
                dst: 1,
                lhs: 20,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::ST,
                dst: 200,
                lhs: 1,
                rhs: 0,
            },
            Instruction {
                opcode: OpCodes::LD,
                dst: 2,
                lhs: 200,
                rhs: 0,
            },
        ];

        let registers = run_program(program);

        assert_eq!(registers, [0, 20, 20, 0, 0, 0, 0, 0]);
    }
}
