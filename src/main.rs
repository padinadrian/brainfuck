//! Interpreter for brainfuck programming language.

/* ========== Includes ========== */
use eyre::Result;
use num_enum::FromPrimitive;
use std::io::{self, Read, Write};

/* ========== Enums ========== */

/// Size of the program memory in bytes.
const PROGRAM_MEMORY_SIZE: usize = 8;

/* ========== Enums ========== */

/// A single instruction of code.
#[derive(Debug, Default, Clone, Copy, PartialEq, FromPrimitive)]
#[repr(u8)]
enum Command {
    /// Unknown command - these are ignored.
    #[default]
    Unknown,
    /// Increment the data pointer by one (to point to the next cell to the right).
    IncrementPointer = b'>',
    /// Decrement the data pointer by one (to point to the previous cell to the right).
    DecrementPointer = b'<',
    /// Increment the byte at the data pointer by one.
    IncrementValue = b'+',
    /// Decrement the byte at the data pointer by one.
    DecrementValue = b'-',
    /// Output the byte at the data pointer.
    OutputValue = b'.',
    /// Accept one byte of input, storing its value in the byte at the data pointer.
    InputValue = b',',
    /// If the byte at the data pointer is zero, then instead of moving the
    /// instruction pointer forward to the next command, jump it forward to the
    /// command after the matching end bracket.
    JumpForward = b'[',
    /// If the byte at the data pointer is nonzero, then instead of moving the
    /// instruction pointer forward to the next command, jump it back to the
    /// command after the matching start bracket.
    JumpBackward = b']',
}

/* ========== Structs ========== */

/// Contains the current state of the program.
#[derive(Debug, Clone)]
struct ProgramState {
    /// Pointer to the currently-active memory address.
    pub data_ptr: usize,

    /// Pointer to the instruction currently being executed.
    pub instruct_ptr: usize,

    /// Current state of the program memory.
    pub memory: [u8; PROGRAM_MEMORY_SIZE],
}

impl ProgramState {
    /// Create a new ProgramState object.
    pub fn new() -> Self {
        let memory = [0u8; PROGRAM_MEMORY_SIZE];
        Self {
            data_ptr: 0,
            instruct_ptr: 0,
            memory,
        }
    }

    /// Given a program command, return a new state equivalent to the current
    /// state with the command applied to it.
    pub fn step(&self, code: &[Command]) -> Result<ProgramState> {
        let mut new_state = self.clone();
        let command = code[new_state.instruct_ptr];

        // println!("State:");
        // println!("  data_ptr: {:08X?}", new_state.data_ptr);
        // println!("  value: {}", new_state.get_value());
        // println!("  instruct_ptr {:08X?}:", new_state.instruct_ptr);
        // println!("  command: {:?}", command);

        match command {
            Command::IncrementPointer => {
                new_state.data_ptr += 1;
            }
            Command::DecrementPointer => {
                new_state.data_ptr -= 1;
            }
            Command::IncrementValue => {
                new_state.memory[new_state.data_ptr] += 1;
            }
            Command::DecrementValue => {
                new_state.memory[new_state.data_ptr] -= 1;
            }
            Command::OutputValue => {
                // Print the raw byte directly to stdout.
                let value = new_state.get_value();
                io::stdout().write_all(&[value])?;
                let _ = io::stdout().flush();
            }
            Command::InputValue => {
                // TODO: Not sure of the best way to accept user input.
                // let mut input = String::new();
                let mut buf = [0];
                let _ = io::stdin().read(&mut buf);
                new_state.memory[new_state.data_ptr] = buf[0];
            }
            Command::JumpForward => {
                let value = new_state.get_value();
                if value == 0 {
                    // Find matching end bracket.
                    let mut instruct_ptr = new_state.instruct_ptr;
                    let end = code.len();
                    let mut bracket_counter = 1;
                    while instruct_ptr < end && bracket_counter > 0 {
                        instruct_ptr += 1;
                        match code[instruct_ptr] {
                            Command::JumpForward => {
                                bracket_counter += 1;
                            }
                            Command::JumpBackward => {
                                bracket_counter -= 1;
                            }
                            _ => {}
                        }
                    }
                    // New instruction is after the end bracket, but this is
                    // handled at the very end of the step function.
                    // println!("New instruct_ptr: {}", instruct_ptr);
                    new_state.instruct_ptr = instruct_ptr;
                }
            }
            Command::JumpBackward => {
                let value = new_state.get_value();
                if value != 0 {
                    // Find the matching start bracket.
                    let mut instruct_ptr = new_state.instruct_ptr;
                    let mut bracket_counter = 1;
                    while bracket_counter > 0 {
                        instruct_ptr -= 1;
                        match code[instruct_ptr] {
                            Command::JumpForward => {
                                bracket_counter -= 1;
                            }
                            Command::JumpBackward => {
                                bracket_counter += 1;
                            }
                            _ => {}
                        }
                    }
                    // New instruction is after the start bracket, but this is
                    // handled at the very end of the step function.
                    // println!("New instruct_ptr: {}", instruct_ptr);
                    new_state.instruct_ptr = instruct_ptr;
                }
            }
            _ => {
                // For any other command, do nothing.
            }
        }

        // Move to the next instruction.
        new_state.instruct_ptr += 1;
        Ok(new_state)
    }

    /// Get the current value of memory pointed to by the data pointer.
    #[inline(always)]
    fn get_value(&self) -> u8 {
        self.memory[self.data_ptr]
    }
}

impl Default for ProgramState {
    fn default() -> Self {
        Self::new()
    }
}

/// The program runner.
#[derive(Debug, Default, Clone)]
struct Program {
    /// The current state of the program.
    state: ProgramState,
}

impl Program {
    /// Create a new Program object.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset the program state back to default.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.state = ProgramState::new();
    }

    /// Run a set of commands to completion.
    pub fn run(&mut self, code: &[Command]) -> Result<()> {
        while self.state.instruct_ptr < code.len() {
            self.state = self.state.step(code)?;
        }
        Ok(())
    }
}

/* ========== Functions ========== */

/// Check brackets in the input code to make sure they are valid.
#[allow(dead_code)]
fn validate_brackets(_code: &[u8]) -> Result<()> {
    // TODO
    Ok(())
}

/* ========== MAIN ========== */

fn main() {
    // Read code
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide the name of the program file.");
        return;
    }

    let filename = &args[1];
    let code: Vec<_> = std::fs::read(filename)
        .expect("Failed to read from program file")
        .iter()
        .filter_map(|b| match Command::from(*b) {
            Command::Unknown => None,
            command => Some(command),
        })
        .collect();

    // println!("Code: {:?}", code);

    // TODO: Validate code

    // Execute code
    let mut program = Program::new();
    let result = program.run(&code);

    if let Err(e) = result {
        println!("\nProgram exited with error: {:?}", e);
    }
}
