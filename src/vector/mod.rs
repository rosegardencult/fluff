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

pub mod vector2;
pub mod vector3;
pub mod vector4;

pub trait Base<U> {
  // Get length of vector by computing sqrt(x^2 + y^2 + ...)
  fn magnitude(&self) -> U;

  // Shrink vector to have magnitude of 1
  fn normalize(&mut self) -> Self;

  // Get dot product of vector and another vector by computing (x * rhs.x + y * rhs.y + ... )
  fn dot(&self, rhs: Self) -> U;
}
