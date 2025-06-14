pub struct Vector4{
  pub w: f32,
  pub x:f32,
  pub y:f32,
  pub z:f32
}

impl Vector4{
  pub fn new()->Vector4{
    Vector4{ w: 0.0,x: 0.0, y: 0.0, z:0.0 }
  }

  pub fn add(vec1:Vector4, vec2:Vector4)->Vector4{
    Vector4{ w: vec1.w + vec2.w, x:vec1.x + vec2.x, y: vec1.y + vec2.y, z: vec1.z + vec2.z }
  }


  pub fn substract(vec1:Vector4, vec2:Vector4)->Vector4{
    Vector4{ w: vec1.w - vec2.w, x:vec1.x - vec2.x, y: vec1.y - vec2.y, z: vec1.z - vec2.z}
  }

}

