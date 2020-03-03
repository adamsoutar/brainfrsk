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

  fn bracket_match (&mut self, backwards: bool) -> Result<usize, &'static str> {
    let start = if backwards { Instruction::LoopEnd } else { Instruction::LoopStart };
    let end = if backwards { Instruction::LoopStart } else { Instruction::LoopEnd };

    let mut ignore = 0;
    let mut r: Vec<usize> = Vec::new();
    if backwards {
      for x in (0..self.ins_ptr).rev() {
        r.push(x);
      }
    } else {
      for x in self.ins_ptr..self.instructions.len() {
        r.push(x);
      }
    }

    for ic in r {
      let val = &self.instructions[ic];
      
      if *val == start {
        ignore += 1;
      }

      if *val == end {
        if ignore == 0 {
          return Ok(ic)
        }
        ignore -= 1;
      }
    };

    Err("Bracket doesn't have a match")
  }

  fn start_loop (&mut self) {
    if self.tape.get_value() != 0 {
      return
    }

    let end = self.bracket_match(false).unwrap();
    self.ins_ptr = end;
  }
  fn end_loop (&mut self) {
    if self.tape.get_value() == 0 {
      return
    }

    let start = self.bracket_match(true).unwrap();
    self.ins_ptr = start;
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
      Instruction::LoopStart => self.start_loop(),
      Instruction::LoopEnd => self.end_loop()
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
