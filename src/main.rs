use chip8emulator::{display, frontend::{crossterm, Frontend}};

fn main() {

  // TODO: clean this up and add commandline parser ( or whatever u call it )
  let ct = crossterm::CrosstermFrontend::new();

  let mut d: display::Display = display::Display::new();

  d.set_pixel(0, 0, true);
  d.set_pixel(1, 1, true);
  d.set_pixel(3, 3, true);
  d.set_pixel(4, 4, true);
  d.set_pixel(3, 5, true);
  d.set_pixel(3, 5, true);

  let _ = ct.draw_display(d.get_display());
  let _ = ct.draw_display(d.get_display());

}
