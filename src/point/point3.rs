// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Three component point representation
//
//===----------------------------------------------------------------------===//

use crate::Component;

use std::fmt;
use std::ops::*;

#[doc = "A point with x and y components"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point3<T: Component> {
  pub x: T,
  pub y: T,
  pub z: T,
}


impl<T: Component + fmt::Display> fmt::Display for Point3<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Point3 - (x:{0}, y:{1}, z:{2})", self.x, self.y, self.z);
  }
}

impl<T: Component> Add for Point3<T> {
  type Output = Point3<T>;

  #[doc = "Add a Point3 to another Point3"]
  fn add(self, rhs: Point3<T>) -> Point3<T> {
    return Point3::<T> {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
      z: self.z + rhs.z,
    };
  }
}


impl<T: Component> Sub<Point3<T>> for Point3<T> {
  type Output = Point3<T>;

  #[doc = "Subtract a Point3 from another Point3"]
  fn sub(self, rhs: Point3<T>) -> Point3<T> {
    return Point3::<T> {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
      z: self.z - rhs.z,
    };
  }
}

impl<T: Component + Neg<Output = T>> Neg for Point3<T> {
  type Output = Point3<T>;

  #[doc = "Reverse the sign of the point's components"]
  fn neg(self) -> Point3<T> {
    return Point3::<T> {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    };
  }
}

impl<T: Component> Mul<T> for Point3<T> {
  type Output = Point3<T>;

  #[doc = "Multiply a Point3 by a scalar"]
  fn mul(self, rhs: T) -> Point3<T> {
    return Point3 {
      x: self.x * rhs,
      y: self.y * rhs,
      z: self.z * rhs,
    };
  }
}

impl<T: Component> Div<T> for Point3<T> {
  type Output = Point3<T>;

  #[doc = "Divide a Point3 by a scalar"]
  fn div(self, rhs: T) -> Point3<T> {
    return Point3 {
      x: self.x / rhs,
      y: self.y / rhs,
      z: self.z / rhs,
    };
  }
}
