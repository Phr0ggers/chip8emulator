use crate::{display::Display, frontend::Frontend, memory::Memory};

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

  pub fn cycle(&mut self, display: &mut Display, memory: &mut Memory, frontend: impl Frontend) {

    // TODO: read input somewhere inbetween and safe to 'input status' also need to do that
    let op_code = memory.fetch(self.pc);

    self.pc += 2;

    let redraw = self.decode_and_execute(op_code, display);

    if redraw {
      let _ = frontend.draw_display(display.get_display());
    }
  }

  fn decode_and_execute(&mut self, op_code: u16, display: &mut Display) -> bool {
    let n1 = (op_code >> 12) & 0xf;

    let mut redraw = false;

    match n1 {
      0x0 => {
        match op_code {
          // clear display
          0x00e0 => {
            display.clear();
            redraw = true;
          },
          _ => ()
        }
      },
      0x1 => {
        // 0x1nnn - jump to addr nnn
        self.pc = op_code & 0xfff;
      },
      0x6 => {
        // 0x6xnn - set Vx to nn
        self.reg[((op_code >> 8) & 0xf) as usize] = (op_code & 0xff) as u8;
      },
      0x7 => {
        //0x7xnn - add nn to Vx
        self.reg[((op_code >> 8) & 0xf) as usize] += (op_code & 0xff) as u8;
      },
      0xa => {
        //0xannn - set i to nnn
        self.i = op_code & 0xfff;
      },
      0xd => {
        //0xdxyn - sets n bytes from memory at i to (Vx, Vy), Vf = collision
        let collision = display.draw_sprite();
        self.reg[0xf] = if collision {1} else {0};
        redraw = true;
      },
      _ => (),

    };
    redraw
  }
}
