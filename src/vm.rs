mod tape;
use crate::instructions::*;
use std::io::{self, BufRead};

fn get_one_line_from_stdin () -> String {
  let stdin = io::stdin();
  let line = stdin.lock().lines().next().unwrap().unwrap();
  line
}

pub struct VM {
  tape: tape::Tape,
  instructions: Vec<Instruction>,
  ins_ptr: usize
}

impl VM {
  fn inc (&mut self) {
    let val = self.tape.get_value();
    self.tape.set_value(val + 1);
  }
  fn dec  (&mut self) {
    let val = self.tape.get_value();
    self.tape.set_value(val - 1);
  }
  fn output (&self) {
    let ch = char::from(self.tape.get_value());
    print!("{}", ch);
  }
  fn input (&mut self) {
    // Read one *line* from stdin.
    // Unfortunately this is *much* easier than char by char (raw TTY)
    let line = get_one_line_from_stdin();
    let chr_byte = line.as_bytes().first().unwrap();
    self.tape.set_value(*chr_byte);
  }

  fn interpret_instruction (&mut self) {
    let i = &self.instructions[self.ins_ptr];
    
    match i {
      Instruction::Left => self.tape.left(),
      Instruction::Right => self.tape.right(),
      Instruction::Increment => self.inc(),
      Instruction::Decrement => self.dec(),
      Instruction::Output => self.output(),
      Instruction::Input => self.input(),
      // TODO: The rest
      _ => ()
    }
  }

  fn run (&mut self) {
    while self.ins_ptr < self.instructions.len() {
      self.interpret_instruction();
      self.ins_ptr += 1;
    }
  }
}

pub fn run_for_instructions(instrs: Vec<Instruction>) {
  let mut vm = VM {
    tape: tape::new_tape(),
    instructions: instrs,
    ins_ptr: 0
  };

  vm.run();
}
