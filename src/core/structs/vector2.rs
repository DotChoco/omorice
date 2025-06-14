pub struct Vector2{
  _x:f32,
  _y:f32
}

impl Default for Vector2 {
  fn default() -> Self { Self { _x: 0.0, _y: 0.0 } }
}

impl Vector2{
  pub fn new(x:f32, y:f32) -> Vector2{ Vector2{ _x: x, _y: y } }

  pub fn get_x(&self)->f32 { self._x }
  pub fn get_y(&self)->f32 { self._y }

  pub fn add(vec1:Vector2, vec2:Vector2)->Vector2{
    Vector2{ _x:vec1.get_x() + vec2.get_x(), _y: vec1.get_y() + vec2.get_y() }
  }

  pub fn substract(vec1:Vector2, vec2:Vector2)->Vector2{
    Vector2{ _x:vec1.get_x() - vec2.get_x(), _y: vec1.get_y() - vec2.get_y() }
  }

  pub fn multiply(vec1:Vector2, vec2:Vector2)->Vector2{
    Vector2{ _x:vec1.get_x() * vec2.get_x(), _y: vec1.get_y() * vec2.get_y() }
  }

  pub fn divide(vec1:Vector2, vec2:Vector2)->Vector2{
    Vector2{ _x:vec1.get_x() / vec2.get_x(), _y: vec1.get_y() / vec2.get_y() }
  }

}

