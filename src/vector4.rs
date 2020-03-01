// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Four point vector representation
//
//===----------------------------------------------------------------------===//

use std::fmt;

pub struct Vector4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl fmt::Display for Vector4 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(
      f,
      "Vector4 - [x:{0}, y:{1}, z:{2}, w:{3}]",
      self.x, self.y, self.z, self.w
    );
  }
}
