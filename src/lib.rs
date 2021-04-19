extern crate palette;
extern crate sdl2;

use palette::{Hsv, LinSrgb};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use std::error::Error;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::thread::sleep;
use std::time::Duration;

const SLEEP_SECOND: u32 = 1_000_000_000;

struct State {
  done: bool,
  fps: u32,
  frame: u64,
  background_color_base: ColorBase,
  hsv_delta: Hsv,
  hsv_offset: Hsv,
  background_color: Hsv,
}

impl State {
  pub fn new() -> Self {
    let fps = 60;
    let background_color_base = ColorBase::Red;
    Self {
      done: false,
      fps,
      frame: 0,
      background_color_base,
      hsv_delta: Hsv::new::<f32>(60.0 / fps as f32, 0.0, 0.0),
      hsv_offset: Hsv::new::<f32>(0.0, 0.0, 0.0),
      background_color: background_color_base.to_hsv(),
    }
  }
}

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
  for message in args {
    println!("Program Argument: {}", message);
  }
  println!("");

  // SDL2 Setup
  let sdl_context = sdl2::init()?;
  let sdl_video_subsystem = sdl_context.video()?;
  let sdl_window_title = "SDL2 Template";
  let sdl_window_width = 800;
  let sdl_window_height = 600;
  let sdl_window = sdl_video_subsystem
    .window(sdl_window_title, sdl_window_width, sdl_window_height)
    .position_centered()
    .resizable()
    .build()?;
  let mut sdl_canvas = sdl_window.into_canvas().build()?;
  let mut sdl_event_pump = sdl_context.event_pump()?;

  // Main Loop
  let mut state = State::new();
  while !state.done {
    input(&mut state, &mut sdl_event_pump);
    update(&mut state);
    output(&mut state, &mut sdl_canvas);
  }
  Ok(())
}

fn input(state: &mut State, sdl_event_pump: &mut sdl2::EventPump) {
  for event in sdl_event_pump.poll_iter() {
    let mut print = true;
    match event {
      Event::Quit { .. }
      | Event::KeyDown {
        keycode: Some(Keycode::Escape),
        ..
      } => state.done = true,
      // skip mouse motion intentionally because of the verbose it might cause.
      Event::MouseMotion { .. } => print = false,
      _ => {},
    }
    if print {
      println!("SDL2 Event: {:?}", event);
    }
  }
}

fn update(state: &mut State) {
  // Background Color Update
  let flash_interval = 7; // seconds between flashes
  let flash_duration = 1; // flash frames
  let flash = state.frame % u64::from(flash_interval * state.fps) < flash_duration;
  if 0 == state.frame % u64::from(flash_interval * state.fps) {
    state.background_color_base = match state.background_color_base {
      ColorBase::Red => ColorBase::Green,
      ColorBase::Green => ColorBase::Blue,
      ColorBase::Blue => ColorBase::Red,
      ColorBase::White => ColorBase::Black,
      ColorBase::Black => ColorBase::White,
    };
  }
  state.hsv_offset = state.hsv_offset + state.hsv_delta;
  state.background_color = match flash {
    true => ColorBase::White.to_hsv(), // flash color
    false => state.background_color_base.to_hsv() + state.hsv_offset,
  };

  // Diagnostic Output
  let print_seconds = 60;
  if 0 == state.frame % u64::from(print_seconds * state.fps) {
    println!("State Update: Frame {} => HSV {:?}",
      state.frame,
      state.background_color
    );
  }

  // Frame Update
  state.frame = state.frame.wrapping_add(1);
}

fn output(state: &mut State, canvas: &mut Canvas<sdl2::video::Window>) {
  canvas.set_draw_color(hsv_to_color(state.background_color));
  canvas.clear();
  canvas.present();
  sleep(Duration::new(0, SLEEP_SECOND / state.fps));
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

