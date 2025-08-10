// src/cpu.rs - registers, timers, cycle

pub struct Cpu {
  // 16 registers V0-VF
  reg: [u8; 16],

  // index register
  i: u16,

  // program counter
  pc: u16,

  // stack pointer
  sp: u8,

  // stack
  stack: [u16; 32],

  // delay timer
  dt: u8,

  // sound timer
  st: u8,

}

impl Cpu {
  pub fn new() -> Self {
    Cpu {
      reg: [0; 16],
      i: 0,
      pc: 0x200,
      sp: 0,
      stack: [0; 32],
      dt: 0,
      st: 0
    }
  }

  pub fn cycle() {

  }
}
