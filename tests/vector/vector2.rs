// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Vector2 test point
//
//===----------------------------------------------------------------------===//

use fluff::Vector2;

#[test]
fn to_string() {
  let v = Vector2 { x: 1, y: 2 };
  assert_eq!("Vector2 - [x:1, y:2]", v.to_string());
}

#[test]
fn compare_equal() {
  let a = Vector2 { x: 2, y: 2 };
  let b = Vector2 { x: 2, y: 2 };
  assert!(a == b);
}

#[test]
fn compare_unequal_x() {
  let a = Vector2 { x: 1, y: 1 };
  let b = Vector2 { x: 2, y: 1 };
  assert!(a != b);
}

#[test]
fn compare_unequal_y() {
  let a = Vector2 { x: 1, y: 1 };
  let b = Vector2 { x: 1, y: 2 };
  assert!(a != b);
}
