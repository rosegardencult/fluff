// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Vector representation entry point
//
//===----------------------------------------------------------------------===//

pub mod vector2;
pub mod vector3;
pub mod vector4;

pub trait Base {
  // Get length of vector from sqrt(x^2 + y^2 + ...)
  fn length(&self) -> i32;

  // Add Base to Vector2
  // Add Base to Vector3
  // Add Base to Vector
}
