// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Two point vector representation
//
//===----------------------------------------------------------------------===//

use std::fmt;

pub struct Vector2 {
  pub x: f32,
  pub y: f32,
}

impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Vector2 - [x:{0}, y:{1}]", self.x, self.y);
  }
}
