// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Vector2 Vector3 Vector4 test point
//
//===----------------------------------------------------------------------===//

use fluff::Vector2;
use fluff::Vector3;
use fluff::Vector4;

#[test]
fn vector_two_to_string() {
  let v = Vector2 { x: 3.14, y: 1.59 };
  assert_eq!("Vector2 - [x:3.14, y:1.59]", v.to_string());
}

#[test]
fn vector_three_to_string() {
  let v = Vector3 {
    x: 3.0,
    y: 2.0,
    z: 1.0,
  };
  assert_eq!("Vector3 - [x:3, y:2, z:1]", v.to_string());
}

#[test]
fn vector_four_to_string() {
  let v = Vector4 {
    x: 1.0,
    y: 2.0,
    z: 3.0,
    w: 4.0,
  };
  assert_eq!("Vector4 - [x:1, y:2, z:3, w:4]", v.to_string());
}
