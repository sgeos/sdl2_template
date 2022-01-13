extern crate palette;
extern crate sdl2;

use clap::{App, Arg};
use palette::{FromColor, Hsv, Srgb};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;

use std::error::Error;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

const APP_NAME: &str = "SDL2 Template";
const APP_VERSION: &str = "0.1";
const SLEEP_SECOND: u32 = 1_000_000_000; // nanoseconds
const DEFAULT_FPS: u32 = 60; // fps
const DEFAULT_SDL_WINDOW_WIDTH: u32 = 800; // pixels
const DEFAULT_SDL_WINDOW_HEIGHT: u32 = 600; // pixels
const DEFAULT_FLASH_INTERVAL: u32 = 7; // seconds
const DEFAULT_FLASH_DURATION: u32 = 1; // frames

struct State {
  done: bool,
  fps: u32,
  frame: u64,
  background_color_base: ColorBase,
  hsv_delta: Hsv,
  hsv_offset: Hsv,
  background_color: Hsv,
  flash_interval: u32,
  flash_duration: u32,
}

impl State {
  pub fn new() -> Self {
    let fps = DEFAULT_FPS;
    let background_color_base = ColorBase::Red;
    Self {
      done: false,
      fps,
      frame: 0,
      background_color_base,
      hsv_delta: Hsv::new::<f32>(60.0 / fps as f32, 0.0, 0.0),
      hsv_offset: Hsv::new::<f32>(0.0, 0.0, 0.0),
      background_color: background_color_base.to_hsv(),
      flash_interval: DEFAULT_FLASH_INTERVAL,
      flash_duration: DEFAULT_FLASH_DURATION,
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
  // Command Line Arguments
  let matches = parse_args(args);
  let mut state = State::new();
  state.fps = match_value(&matches, "fps", DEFAULT_FPS);
  state.flash_interval =
    match_value(&matches, "flash_interval", DEFAULT_FLASH_INTERVAL);
  state.flash_duration =
    match_value(&matches, "flash_duration", DEFAULT_FLASH_DURATION);
  let sdl_window_title = matches.value_of("sdl_window_title").unwrap();
  let sdl_window_width =
    match_value(&matches, "sdl_window_width", DEFAULT_SDL_WINDOW_WIDTH);
  let sdl_window_height =
    match_value(&matches, "sdl_window_height", DEFAULT_SDL_WINDOW_HEIGHT);
  // --fullscreen flag or FULLSCREEN=true environment variable
  let sdl_window_fullscreen =
    matches.is_present("sdl_window_fullscreen") ||
    match_value(&matches, "env_sdl_window_fullscreen", false);
  println!("SDL Window Title: {}", sdl_window_title);
  println!("SDL Window Width: {} pixels", sdl_window_width);
  println!("SDL Window Height: {} pixels", sdl_window_height);
  println!("SDL Window Fullscreen: {}", sdl_window_fullscreen);
  println!("FPS: {}", state.fps);
  println!("Flash Interval: {} seconds", state.flash_interval);
  println!("Flash Duration: {} frames", state.flash_duration);

  // SDL2 Setup
  let sdl_context = sdl2::init()?;
  let sdl_video_subsystem = sdl_context.video()?;
  let mut sdl_window = sdl_video_subsystem
    .window(sdl_window_title, sdl_window_width, sdl_window_height)
    .position_centered()
    .resizable()
    .build()?;
  if sdl_window_fullscreen {
    sdl_window.set_fullscreen(sdl2::video::FullscreenType::Desktop)?;
  }
  let mut sdl_canvas = sdl_window.into_canvas().build()?;
  let mut sdl_event_pump = sdl_context.event_pump()?;

  // Main Loop
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
  let flash_timer = state.frame % u64::from(state.flash_interval * state.fps);
  let flash = flash_timer < state.flash_duration.into();
  if 0 == flash_timer {
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
  let rgb = Srgb::from_color(hsv).into_linear();
  let r: u8 = (255.0 * rgb.red) as u8;
  let g: u8 = (255.0 * rgb.green) as u8;
  let b: u8 = (255.0 * rgb.blue) as u8;
  let a: u8 = 255;
  Color::RGBA(r, g, b, a)
}

fn parse_args(args: Vec<&str>) -> clap::ArgMatches {
  App::new(APP_NAME)
    .version(APP_VERSION)
    .author("Brendan Sechter <sgeos@hotmail.com>")
    .about("SDL2 template project.")
    .arg(Arg::new("sdl_window_title")
      .env("WINDOW_TITLE")
      .help("Window title.")
      .long("title")
      .short('t')
      .takes_value(true)
      .default_value(APP_NAME)
    )
    .arg(Arg::new("sdl_window_width")
      .env("WINDOW_WIDTH")
      .help("Window width.")
      .long("width")
      .short('w')
      .takes_value(true)
      .default_value(&DEFAULT_SDL_WINDOW_WIDTH.to_string())
    )
    .arg(Arg::new("sdl_window_height")
      .env("WINDOW_HEIGHT")
      .help("Window height.")
      .long("height")
      .short('h')
      .takes_value(true)
      .default_value(&DEFAULT_SDL_WINDOW_HEIGHT.to_string())
    )
    // --fullscreen flag or FULLSCREEN=true environment variable
    .arg(Arg::new("sdl_window_fullscreen")
      .help("Launch in fullscreen mode.")
      .long("fullscreen")
    )
    .arg(Arg::new("env_sdl_window_fullscreen")
      .env("FULLSCREEN")
      .help("Launch in fullscreen mode.")
      .default_value("false")
    )
    .arg(Arg::new("fps")
      .env("FPS")
      .help("Target FPS.")
      .long("fps")
      .short('f')
      .takes_value(true)
      .default_value(&DEFAULT_FPS.to_string())
    )
    .arg(Arg::new("flash_interval")
      .env("FLASH_INTERVAL")
      .help("Flash interval in seconds.")
      .long("interval")
      .short('i')
      .takes_value(true)
      .default_value(&DEFAULT_FLASH_INTERVAL.to_string())
    )
    .arg(Arg::new("flash_duration")
      .env("FLASH_DURATION")
      .help("Flash duration in frames.")
      .long("duration")
      .short('d')
      .takes_value(true)
      .default_value(&DEFAULT_FLASH_DURATION.to_string())
    )
    .get_matches_from(args)
}

fn match_value<T: FromStr>(matches: &clap::ArgMatches, key: &str, default: T) -> T {
  println!("{} {:?}", key, matches.value_of(key));
  matches
    .value_of(key)
    .unwrap()
    .parse::<T>()
    .unwrap_or(default)
}
 
