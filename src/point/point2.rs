// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Two component point representation
//
//===----------------------------------------------------------------------===//

use crate::Component;

use std::fmt;
use std::ops::*;

#[doc = "A point with x and y components"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Point2<T: Component> {
  pub x: T,
  pub y: T,
}

impl<T: Component + fmt::Display> fmt::Display for Point2<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "Point2 - (x:{0}, y:{1})", self.x, self.y);
  }
}

impl<T: Component> Add for Point2<T> {
  type Output = Point2<T>;

  #[doc = "Add a Point2 to another Point2"]
  fn add(self, rhs: Point2<T>) -> Point2<T> {
    return Point2::<T> {
      x: self.x + rhs.x,
      y: self.y + rhs.y,
    };
  }
}


impl<T: Component> Sub<Point2<T>> for Point2<T> {
  type Output = Point2<T>;

  #[doc = "Subtract a Point2 from another Point2"]
  fn sub(self, rhs: Point2<T>) -> Point2<T> {
    return Point2::<T> {
      x: self.x - rhs.x,
      y: self.y - rhs.y,
    };
  }
}

impl<T: Component + Neg<Output = T>> Neg for Point2<T> {
  type Output = Point2<T>;

  #[doc = "Reverse the sign of the point's components"]
  fn neg(self) -> Point2<T> {
    return Point2::<T> {
      x: -self.x,
      y: -self.y,
    };
  }
}

impl<T: Component> Mul<T> for Point2<T> {
  type Output = Point2<T>;

  #[doc = "Multiply a Point2 by a scalar"]
  fn mul(self, rhs: T) -> Point2<T> {
    return Point2 {
      x: self.x * rhs,
      y: self.y * rhs,
    };
  }
}

impl<T: Component> Div<T> for Point2<T> {
  type Output = Point2<T>;

  #[doc = "Divide a Point2 by a scalar"]
  fn div(self, rhs: T) -> Point2<T> {
    return Point2 {
      x: self.x / rhs,
      y: self.y / rhs,
    };
  }
}
