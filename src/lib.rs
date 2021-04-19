extern crate palette;
extern crate sdl2;

use palette::{Hsv, LinSrgb};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::error::Error;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::thread::sleep;
use std::time::Duration;

const SLEEP_SECOND: u32 = 1_000_000_000;

#[derive(Clone, Copy)]
enum ColorBase {
  Black,
  White,
  Red,
  Green,
  Blue,
}

impl ColorBase {
  fn to_hsv(self) -> Hsv {
    match self {
      ColorBase::Black => Hsv::new::<f32>(0.0, 0.0, 0.0),
      ColorBase::White => Hsv::new::<f32>(0.0, 0.0, 1.0),
      ColorBase::Red => Hsv::new::<f32>(0.0, 1.0, 1.0),
      ColorBase::Green => Hsv::new::<f32>(120.0, 1.0, 1.0),
      ColorBase::Blue => Hsv::new::<f32>(240.0, 1.0, 1.0),
    }
  }
}

#[no_mangle]
pub extern fn run(argc: c_int, argv: *const *const c_char) {
  let mut args = Vec::new();
  for i in 0..(argc as isize) {
    unsafe {
      let arg: &str = CStr::from_ptr(*argv.offset(i)).to_str().unwrap_or("");
      args.push(arg);
    }
  }
  match rlib_run(args) {
    Err(e) => println!("{:?}", e),
    _ => ()
  }
}

pub fn rlib_run(args: Vec<&str>) -> Result<(), Box<dyn Error>> {
  // Arguments
  println!("Arguments:");
  for message in args {
    println!("  {}", message);
  }
  println!("");

  // SDL2 Setup
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let window_title = "SDL2 Template";
  let window_width = 800;
  let window_height = 600;
  let window = video_subsystem
    .window(window_title, window_width, window_height)
    .position_centered()
    .resizable()
    .build()?;
  let mut canvas = window.into_canvas().build()?;

  // Program State
  let mut done = false;
  let fps = 60;
  let mut frame: u64 = 0;
  let mut background_color_base = ColorBase::Red;
  // hue degrees per second
  let hsv_delta = Hsv::new::<f32>(60.0 / fps as f32, 0.0, 0.0);
  let mut hsv_offset = Hsv::new::<f32>(0.0, 0.0, 0.0);

  // Main Loop
  println!("SDL2 events:");
  let mut event_pump = sdl_context.event_pump()?;
  while !done {
    // Process Events
    for event in event_pump.poll_iter() {
      let mut print = true;
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => done = true,
        // skip mouse motion intentionally because of the verbose it might cause.
        Event::MouseMotion { .. } => print = false,
        _ => {},
      }
      if print {
        println!("{:?}", event);
      }
    }

    // Update
    let flash_interval = 7; // seconds between flashes
    let flash_duration = 1; // flash frames
    let flash = frame % u64::from(flash_interval * fps) < flash_duration;
    if 0 == frame % u64::from(flash_interval * fps) {
      background_color_base = match background_color_base {
        ColorBase::Red => ColorBase::Green,
        ColorBase::Green => ColorBase::Blue,
        ColorBase::Blue => ColorBase::Red,
        ColorBase::White => ColorBase::Black,
        ColorBase::Black => ColorBase::White,
      };
    }
    hsv_offset = hsv_offset + hsv_delta;
    let background_color = match flash {
      true => ColorBase::White.to_hsv(), // flash color
      false => background_color_base.to_hsv() + hsv_offset,
    };

    // Draw
    canvas.set_draw_color(hsv_to_color(background_color));
    canvas.clear();
    canvas.present();
    sleep(Duration::new(0, SLEEP_SECOND / fps));

    // Diagnostic Output
    let print_seconds = 60;
    if 0 == frame % u64::from(print_seconds * fps) {
      println!("Frame {} / HSV {:?}", frame, background_color);
    }

    // Frame Update
    frame = frame.wrapping_add(1);
  }

  // Done
  Ok(())
}

// Convert from palette HSV to RGB color.
fn hsv_to_color(hsv: Hsv) -> Color {
  let rgb = LinSrgb::from(hsv);
  let r: u8 = (255.0 * rgb.red) as u8;
  let g: u8 = (255.0 * rgb.green) as u8;
  let b: u8 = (255.0 * rgb.blue) as u8;
  let a: u8 = 255;
  Color::RGBA(r, g, b, a)
}

