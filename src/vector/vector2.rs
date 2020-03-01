// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Two point vector representation
//
//===----------------------------------------------------------------------===//

use std::fmt;

use crate::vector::Base;

#[derive(Default)]
pub struct Vector2 {
  pub x: i32,
  pub y: i32,
}

impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Vector2 - [x:{0}, y:{1}]", self.x, self.y);
  }
}

impl Vector2 {}

impl Base for Vector2 {
  fn length(&self) -> i32 {
    return self.x.pow(2) + self.y.pow(2);
  }
}

impl PartialEq for Vector2 {
  fn eq(&self, rhs: &Self) -> bool {
    return self.x == rhs.x && self.y == rhs.y;
  }
}
