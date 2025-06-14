use crate::core::structs::vector4;

pub struct ColorRGBA{
  pub r:i32,
  pub g:i32,
  pub b:i32,
  pub a:i32,
}

impl ColorRGBA {
  pub fn new() -> ColorRGBA { ColorRGBA { r:255,g:255,b:255,a:255 } }
}


pub struct ColorRGB{
  pub r:i32,
  pub g:i32,
  pub b:i32,
}

impl ColorRGB {
  pub fn new() -> ColorRGB { ColorRGB { r:255,g:255,b:255 } }
  // pub fn
}




