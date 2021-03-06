// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Two point vector representation
//
//===----------------------------------------------------------------------===//

use crate::vector::Base;
use crate::Component;

use std::fmt;
use std::ops::*;

#[doc = "A vector with x and y components"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Vector2<T: Component> {
  pub x: T,
  pub y: T,
}

impl<T: Component> Base<T> for Vector2<T> {
  #[doc = "Return the magnitude of the vector, sqrt(x^2 + y^2)"]
  fn magnitude(&self) -> T {
    return (self.x * self.x + self.y * self.y).sqrt();
  }

  #[doc = "Shrink vector to have magnitude of 1"]
  fn normalize(&mut self) -> Vector2<T> {
    *self = *self / self.magnitude();
    return *self;
  }

  #[doc = "Return dot product of the vector and another Vector2"]
  fn dot(&self, rhs: Vector2<T>) -> T {
    return self.x * rhs.x + self.y * rhs.y;
  }
}

impl<T: Component + fmt::Display> fmt::Display for Vector2<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Vector2 - (x:{0}, y:{1})", self.x, self.y);
  }
}

impl<T: Component> Add<Vector2<T>> for Vector2<T> {
  type Output = Vector2<T>;

  #[doc = "Add a Vector2 to another Vector2"]
  fn add(self, rhs: Vector2<T>) -> Vector2<T> {
    return Vector2::<T> {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    };
  }
}

impl<T: Component> Sub<Vector2<T>> for Vector2<T> {
  type Output = Vector2<T>;

  #[doc = "Subtract a Vector2 from another Vector2"]
  fn sub(self, rhs: Vector2<T>) -> Vector2<T> {
    return Vector2::<T> {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }
}

impl<T: Component + Neg<Output = T>> Neg for Vector2<T> {
  type Output = Vector2<T>;

  #[doc = "Reverse the sign of the vector's components"]
  fn neg(self) -> Vector2<T> {
    return Vector2::<T> {
      x: -self.x,
      y: -self.y,
    };
  }
}

impl<T: Component> Mul<T> for Vector2<T> {
  type Output = Vector2<T>;

  #[doc = "Multiply a Vector2 by a scalar"]
  fn mul(self, rhs: T) -> Vector2<T> {
    return Vector2 {
      x: self.x * rhs,
      y: self.y * rhs,
    };
  }
}

impl<T: Component> Div<T> for Vector2<T> {
  type Output = Vector2<T>;

  #[doc = "Divide a Vector2 by a scalar"]
  fn div(self, rhs: T) -> Vector2<T> {
    return Vector2 {
      x: self.x / rhs,
      y: self.y / rhs,
    };
  }
}
