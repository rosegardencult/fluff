// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Three point vector representation
//
//===----------------------------------------------------------------------===//

use std::fmt;

pub struct Vector3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Vector3 - [x:{0}, y:{1}, z:{2}]", self.x, self.y, self.z);
  }
}
