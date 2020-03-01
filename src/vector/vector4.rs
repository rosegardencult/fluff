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
  pub x: i32,
  pub y: i32,
  pub z: i32,
  pub w: i32,
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
