// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Point2 test point
//
//===----------------------------------------------------------------------===//

use fluff::Point2;

#[test]
fn to_string() {
  let a = Point2 { x: 1.0, y: 2.0 };
  assert_eq!("Point2 - (x:1, y:2)", a.to_string());
}

#[test]
fn compare_equal() {
  let a = Point2 { x: 1.0, y: 2.0 };
  let b = Point2 { x: 1.0, y: 2.0 };
  assert!(a == b);
}

#[test]
fn compare_unequal_x() {
  let a = Point2 { x: 1.0, y: 2.0 };
  let b = Point2 { x: 2.0, y: 2.0 };
  assert!(a != b);
}

#[test]
fn compare_unequal_y() {
  let a = Point2 { x: 1.0, y: 2.0 };
  let b = Point2 { x: 1.0, y: 3.0 };
  assert!(a != b);
}

#[test]
fn add_point2() {
  let a = Point2 { x: 1.0, y: 1.0 };
  let b = Point2 { x: 2.0, y: 2.0 };

  assert_eq!(Point2 { x: 3.0, y: 3.0 }, a + b);
}

#[test]
fn sub_point2() {
  let a = Point2 { x: 1.0, y: 1.0 };
  let b = Point2 { x: 2.0, y: 2.0 };

  assert_eq!(Point2 { x: 3.0, y: 3.0 }, a + b);
}

#[test]
fn inverse_positive() {
  let a = Point2 { x: 1.0, y: 1.0 };

  assert_eq!(Point2 { x: -1.0, y: -1.0 }, -a);
}

#[test]
fn inverse_negative() {
  let a = Point2 { x: -1.0, y: -1.0 };

  assert_eq!(Point2 { x: 1.0, y: 1.0 }, -a);
}

#[test]
fn multiply_by_scalar() {
  let a = Point2 { x: 1.0, y: 1.0 };

  assert_eq!(Point2 { x: 2.0, y: 2.0 }, a * 2.0);
}

#[test]
fn divide_by_scalar() {
  let a = Point2 { x: 2.0, y: 2.0 };

  assert_eq!(Point2 { x: 1.0, y: 1.0 }, a / 2.0);
}
