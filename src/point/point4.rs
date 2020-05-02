// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Four component point representation
//
//===----------------------------------------------------------------------===//

use crate::Component;

use std::fmt;
use std::ops::*;

#[doc = "A point with x and y components"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point4<T: Component> {
  pub x: T,
  pub y: T,
  pub z: T,
  pub w: T,
}

impl<T: Component + fmt::Display> fmt::Display for Point4<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(
      f,
      "Point4 - (x:{0}, y:{1}, z:{2}, w:{3})",
      self.x, self.y, self.z, self.w
    );
  }
}

impl<T: Component> Add for Point4<T> {
  type Output = Point4<T>;

  #[doc = "Add a Point4 to another Point4"]
  fn add(self, rhs: Point4<T>) -> Point4<T> {
    return Point4::<T> {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
      w: self.w + rhs.w,
    };
  }
}

impl<T: Component> Sub<Point4<T>> for Point4<T> {
  type Output = Point4<T>;

  #[doc = "Subtract a Point4 from another Point4"]
  fn sub(self, rhs: Point4<T>) -> Point4<T> {
    return Point4::<T> {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
      w: self.w - rhs.w,
    };
  }
}

impl<T: Component + Neg<Output = T>> Neg for Point4<T> {
  type Output = Point4<T>;

  #[doc = "Reverse the sign of the point's components"]
  fn neg(self) -> Point4<T> {
    return Point4::<T> {
      x: -self.x,
      y: -self.y,
      z: -self.z,
      w: -self.w,
    };
  }
}

impl<T: Component> Mul<T> for Point4<T> {
  type Output = Point4<T>;

  #[doc = "Multiply a Point4 by a scalar"]
  fn mul(self, rhs: T) -> Point4<T> {
    return Point4 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
      w: self.w * rhs,
    };
  }
}

impl<T: Component> Div<T> for Point4<T> {
  type Output = Point4<T>;

  #[doc = "Divide a Point4 by a scalar"]
  fn div(self, rhs: T) -> Point4<T> {
    return Point4 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
      w: self.w / rhs,
    };
  }
}
