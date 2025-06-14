pub struct Vector3{
  pub x:f32,
  pub y:f32,
  pub z:f32
}

impl Vector3{
  pub fn new()->Vector3{
    Vector3{ x: 0.0, y: 0.0, z:0.0 }
  }

  pub fn add(vec1:Vector3, vec2:Vector3)->Vector3{
    Vector3{ x:vec1.x + vec2.x, y: vec1.y + vec2.y, z: vec1.z + vec2.z }
  }


  pub fn substract(vec1:Vector3, vec2:Vector3)->Vector3{
    Vector3{ x:vec1.x - vec2.x, y: vec1.y - vec2.y, z: vec1.z - vec2.z}
  }


}

