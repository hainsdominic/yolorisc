# YoloRISC

A simple very reduced instruction set computer virtual machine written in Rust.

## Usage

```bash
cargo run --release -- <file>
```

## Example

```bash
cargo run --release -- programs/sum.yolo
```

## Instruction set

| Instruction | Opcode | Description                                            |
| ----------- | ------ | ------------------------------------------------------ |
| `ADD`       | `0`    | Add two registers                                      |
| `SUB`       | `1`    | Subtract two registers                                 |
| `AND`       | `2`    | Bitwise AND two registers                              |
| `OR`        | `3`    | Bitwise OR two registers                               |
| `XOR`       | `4`    | Bitwise XOR two registers                              |
| `NOT`       | `5`    | Bitwise NOT a register                                 |
| `MOV`       | `6`    | Move a value to a register                             |
| `PRT`       | `7`    | Print a register                                       |
| `END`       | `8`    | End the program                                        |
| `ST`        | `9`    | Store a register to memory                             |
| `LD`        | `10`   | Load a register from memory                            |
| `JZ`        | `11`   | Jump to an instruction index if a register is zero     |
| `JNZ`       | `12`   | Jump to an instruction index if a register is not zero |
| `JMP`       | `13`   | Jump to an instruction index                           |
