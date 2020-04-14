// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Vector representation entry point
//
//===----------------------------------------------------------------------===//

use std::ops::*;

pub mod vector2;
pub mod vector3;
pub mod vector4;

// TODO: Vector of ints?
// TODO: Restrict T to just numeric types
// TODO: Remove requirement of x and y being the same type

pub trait Base<U> {
  // Get length of vector by computing sqrt(x^2 + y^2 + ...)
  fn magnitude(&self) -> U;

  // Shrink vector to have magnitude of 1
  fn normalize(&mut self) -> Self;

  //
  fn dot(&self) {}
}

pub trait Component: Add + Sub + Mul + Copy + Neg + num::Float {} // num::Num?

impl<T> Component for T where T: Add + Sub + Mul + Copy + Neg + num::Float {}
