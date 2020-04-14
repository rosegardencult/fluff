// Copyright (c) 2020 rose garden cult All rights reserved.
// See https://rosegardencult.com/licenses for license information.
//
//===----------------------------------------------------------------------===//
//
// Vector3 test point
//
//===----------------------------------------------------------------------===//

use fluff::Vector3;

#[test]
fn to_string() {
  let v = Vector3 {
    x: 3,
    y: 2,
    z: 1,
  };
  assert_eq!("Vector3 - (x:3, y:2, z:1)", v.to_string());
}
