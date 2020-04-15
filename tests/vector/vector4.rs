// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
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
