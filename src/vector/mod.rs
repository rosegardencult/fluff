// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
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

// TODO: Vector of ints? and then remove requirement of x and y being the same type
// TODO: Restrict T to just numeric types

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
