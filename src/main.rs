//Mods
mod core;
mod ui;

//Uses
use crossterm::{cursor::MoveTo, execute, terminal::{Clear, ClearType}};
use winit::{event, event_loop::{self, EventLoopBuilder}};
use std::io::stdout;
use core::structs::{vector2::*, vector3::*, vector4::*};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};


fn clear_console() {
  execute!(
    stdout(),
    Clear(ClearType::All),
    Clear(ClearType::Purge),
    MoveTo(0,0)
  ).unwrap();
}

fn main() {
  clear_console();

  let mut data = Vector2::default();
  println!("vec in x:{} and y:{}", data.get_x(), data.get_y());

  data = Vector2::new(10., 9.5);
  println!("vec now in x:{} and y:{}", data.get_x(), data.get_y());

  // Crea un "stream" de salida de audio
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();

  // Crea un sink para controlar la reproducción
  let sink = Sink::try_new(&stream_handle).unwrap();

  // Abre el archivo de audio
  let path = "D:/Github/Rust/omurice/assets/Music/SoYou(Korean_RomCom).ogg";
  // let path = "D:/Github/Rust/omurice/assets/Music/Got-me-dancing.ogg";
  let file = File::open(path).unwrap();
  let source = Decoder::new(BufReader::new(file)).unwrap();


  env_logger::init();
  // let event_loop:EventLoopBuilder<CustomEvent> = EventLoopBuilder::<CustomEvent>::with_user_event()
    // .build()
    // .unwrap();


  // Reproduce
  sink.append(source);
  sink.sleep_until_end();  // Espera hasta que termine de reproducir

}



