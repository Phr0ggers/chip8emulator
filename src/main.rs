use chip8emulator::{display, frontend::{crossterm, Frontend}};

fn main() {

  // TODO: clean this up and add commandline parser ( or whatever u call it )
  let ct = crossterm::CrosstermFrontend::new();

  let mut d: display::Display = display::Display::new();

  d.draw_seal();

  let _ = ct.draw_display(d.get_display());

}
