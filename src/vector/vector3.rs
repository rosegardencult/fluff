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
  pub x: i32,
  pub y: i32,
  pub z: i32,
}

impl fmt::Display for Vector3 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Vector3 - [x:{0}, y:{1}, z:{2}]", self.x, self.y, self.z);
  }
}
