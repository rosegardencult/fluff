// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Vector2 test point
//
//===----------------------------------------------------------------------===//

use fluff::vector::Base;
use fluff::Vector2;

#[test]
fn to_string() {
  let v = Vector2 { x: 1.0, y: 2.0 };
  assert_eq!("Vector2 - (x:1, y:2)", v.to_string());
}

#[test]
fn compare_equal() {
  let a = Vector2 { x: 2.0, y: 2.0 };
  let b = Vector2 { x: 2.0, y: 2.0 };
  assert!(a == b);
}

#[test]
fn compare_unequal_x() {
  let a = Vector2 { x: 1.0, y: 1.0 };
  let b = Vector2 { x: 2.0, y: 1.0 };
  assert!(a != b);
}

#[test]
fn compare_unequal_y() {
  let a = Vector2 { x: 1.0, y: 1.0 };
  let b = Vector2 { x: 1.0, y: 2.0 };
  assert!(a != b);
}

#[test]
fn add_vector2() {
  let a = Vector2 { x: 1.0, y: 1.0 };
  let b = Vector2 { x: 2.0, y: 2.0 };

  assert_eq!(Vector2 { x: 3.0, y: 3.0 }, a + b);
}

#[test]
fn sub_vector2() {
  let a = Vector2 { x: 1.0, y: 1.0 };
  let b = Vector2 { x: 2.0, y: 2.0 };

  assert_eq!(Vector2 { x: -1.0, y: -1.0 }, a - b);
}

#[test]
fn inverse_positive() {
  let a = Vector2 { x: 1.0, y: 1.0 };

  assert_eq!(Vector2 { x: -1.0, y: -1.0 }, -a);
}

#[test]
fn inverse_negative() {
  let a = Vector2 { x: -1.0, y: -1.0 };

  assert_eq!(Vector2 { x: 1.0, y: 1.0 }, -a);
}

#[test]
fn multiply_by_scalar() {
  let a = Vector2 { x: 4.0, y: 4.0 };

  assert_eq!(Vector2 { x: 20.0, y: 20.0 }, a * 5.0);
}

#[test]
fn divide_by_scalar() {
  let a = Vector2 { x: 20.0, y: 20.0 };

  assert_eq!(Vector2 { x: 4.0, y: 4.0 }, a / 5.0);
}

#[test]
fn get_magnitude() {
  let a = Vector2 { x: 3.0, y: 1.0 };

  // a.magnitude() should be 3.16227
  // we test the return value of magnitude within 0.00001 of the correct answer
  // because floating point math doesn't need to be any more precise for us
  assert!(a.magnitude() < 3.16228);
  assert!(a.magnitude() > 3.16227);
}

#[test]
fn get_magnitude_zero_vector() {
  let a = Vector2 { x: 0.0, y: 0.0 };

  assert_eq!(a.magnitude(), 0.0);
}

#[test]
fn get_normalize() {
  let a = Vector2::normalize(&mut Vector2 { x: 3.0, y: 1.0 });

  // a.normalize() should be { 0.94868, 0.72547 }
  // we test x and y to be within 0.00001 of the correct answer because
  // floating point math doesn't need to be any more precise for us
  assert!(a.x < 0.94869);
  assert!(a.x > 0.94868);
  assert!(a.y < 0.72548);
  assert!(a.y > 0.72547);
}
