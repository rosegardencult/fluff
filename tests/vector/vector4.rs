// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Vector4 test point
//
//===----------------------------------------------------------------------===//

use fluff::Vector4;

#[test]
fn to_string() {
  let v = Vector4 {
    x: 1,
    y: 2,
    z: 3,
    w: 4,
  };
  assert_eq!("Vector4 - (x:1, y:2, z:3, w:4)", v.to_string());
}
