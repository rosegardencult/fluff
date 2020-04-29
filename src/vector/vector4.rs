// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Four point vector representation
//
//===----------------------------------------------------------------------===//

use crate::vector::Base;
use crate::vector::Component;

use std::fmt;
use std::ops::*;

#[doc = "A vector with x, y, z, and w components"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector4<T: Component> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T: Component> Base<T> for Vector4<T> {
  #[doc = "Return the magnitude of the vector, sqrt(x^2 + y^2 + z^2 + w^2)"]
  fn magnitude(&self) -> T {
    return (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
  }

  #[doc = "Shrink vector to have magnitude of 1"]
  fn normalize(&mut self) -> Vector4<T> {
    *self = *self / self.magnitude();
    return *self;
  }

  #[doc = "Return dot product of the vector and another Vector4"]
  fn dot(&self, rhs: Vector4<T>) -> T {
    return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w;
  }
}

impl<T: Component + fmt::Display> fmt::Display for Vector4<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(
      f,
      "Vector4 - (x:{0}, y:{1}, z:{2}, w:{3})",
      self.x, self.y, self.z, self.w
    );
  }
}

impl<T: Component> Add<Vector4<T>> for Vector4<T> {
  type Output = Vector4<T>;

  #[doc = "Add a Vector4 to another Vector4"]
  fn add(self, rhs: Vector4<T>) -> Vector4<T> {
    return Vector4::<T> {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    };
  }
}

impl<T: Component> Sub<Vector4<T>> for Vector4<T> {
  type Output = Vector4<T>;

  #[doc = "Subtract a Vector4 from another Vector4"]
  fn sub(self, rhs: Vector4<T>) -> Vector4<T> {
    return Vector4::<T> {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    };
  }
}

impl<T: Component + Neg<Output = T>> Neg for Vector4<T> {
  type Output = Vector4<T>;

  #[doc = "Reverse the sign of the vector's components"]
  fn neg(self) -> Vector4<T> {
    return Vector4::<T> {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    };
  }
}

impl<T: Component> Mul<T> for Vector4<T> {
  type Output = Vector4<T>;

  #[doc = "Multiply a Vector4 by a scalar"]
  fn mul(self, rhs: T) -> Vector4<T> {
    return Vector4 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs,
    };
  }
}

impl<T: Component> Div<T> for Vector4<T> {
  type Output = Vector4<T>;

  #[doc = "Divide a Vector4 by a scalar"]
  fn div(self, rhs: T) -> Vector4<T> {
    return Vector4 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      w: self.w / rhs,
    };
  }
}
