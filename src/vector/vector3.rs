// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Three point vector representation
//
//===----------------------------------------------------------------------===//

use crate::vector::Base;
use crate::Component;

use std::fmt;
use std::ops::*;

#[doc = "A vector with x, y and z components"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector3<T: Component> {
  pub x: T,
  pub y: T,
  pub z: T,
}

impl<T: Component> Vector3<T> {
  #[doc = "Return cross product of the vector and another Vector3"]
  pub fn cross(&self, rhs: Vector3<T>) -> Vector3<T> {
    return Vector3::<T> {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    };
  }
}

impl<T: Component> Base<T> for Vector3<T> {
  #[doc = "Return the magnitude of the vector, sqrt(x^2 + y^2 + z^2)"]
  fn magnitude(&self) -> T {
    return (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
  }

  #[doc = "Shrink vector to have magnitude of 1"]
  fn normalize(&mut self) -> Vector3<T> {
    *self = *self / self.magnitude();
    return *self;
  }

  #[doc = "Return dot product of the vector and another Vector3"]
  fn dot(&self, rhs: Vector3<T>) -> T {
    return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
  }
}

impl<T: Component + fmt::Display> fmt::Display for Vector3<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Vector3 - (x:{0}, y:{1}, z:{2})", self.x, self.y, self.z);
  }
}

impl<T: Component> Add<Vector3<T>> for Vector3<T> {
  type Output = Vector3<T>;

  #[doc = "Add a Vector3 to another Vector3"]
  fn add(self, rhs: Vector3<T>) -> Vector3<T> {
    return Vector3::<T> {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    };
  }
}

impl<T: Component> Sub<Vector3<T>> for Vector3<T> {
  type Output = Vector3<T>;

  #[doc = "Subtract a Vector3 from another Vector3"]
  fn sub(self, rhs: Vector3<T>) -> Vector3<T> {
    return Vector3::<T> {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    };
  }
}

impl<T: Component + Neg<Output = T>> Neg for Vector3<T> {
  type Output = Vector3<T>;

  #[doc = "Reverse the sign of the vector's components"]
  fn neg(self) -> Vector3<T> {
    return Vector3::<T> {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    };
  }
}

impl<T: Component> Mul<T> for Vector3<T> {
  type Output = Vector3<T>;

  #[doc = "Multiply a Vector3 by a scalar"]
  fn mul(self, rhs: T) -> Vector3<T> {
    return Vector3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    };
  }
}

impl<T: Component> Div<T> for Vector3<T> {
  type Output = Vector3<T>;

  #[doc = "Divide a Vector3 by a scalar"]
  fn div(self, rhs: T) -> Vector3<T> {
    return Vector3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    };
  }
}
