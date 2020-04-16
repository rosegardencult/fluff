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

use fluff::vector::Base;
use fluff::Vector4;

#[test]
fn to_string() {
  let v = Vector4 {
    x: 3.0,
    y: 2.0,
    z: 1.0,
    w: 0.0,
  };
  assert_eq!("Vector4 - (x:3, y:2, z:1, w:0)", v.to_string());
}

#[test]
fn compare_equal() {
  let a = Vector4 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
    w: 2.0,
  };
  let b = Vector4 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
    w: 2.0,
  };
  assert!(a == b);
}

#[test]
fn compare_unequal_x() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  let b = Vector4 {
    x: 2.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  assert!(a != b);
}

#[test]
fn compare_unequal_y() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  let b = Vector4 {
    x: 1.0,
    y: 2.0,
    z: 1.0,
    w: 1.0,
  };
  assert!(a != b);
}

#[test]
fn compare_unequal_z() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  let b = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 2.0,
    w: 1.0,
  };
  assert!(a != b);
}

#[test]
fn compare_unequal_w() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  let b = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 2.0,
  };
  assert!(a != b);
}

#[test]
fn add_vector4() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  let b = Vector4 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
    w: 2.0,
  };

  assert_eq!(
    Vector4 {
      x: 3.0,
      y: 3.0,
      z: 3.0,
      w: 3.0,
    },
    a + b
  );
}

#[test]
fn sub_vector4() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };
  let b = Vector4 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
    w: 2.0,
  };

  assert_eq!(
    Vector4 {
      x: -1.0,
      y: -1.0,
      z: -1.0,
      w: -1.0,
    },
    a - b
  );
}

#[test]
fn inverse_positive() {
  let a = Vector4 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
    w: 1.0,
  };

  assert_eq!(
    Vector4 {
      x: -1.0,
      y: -1.0,
      z: -1.0,
      w: -1.0,
    },
    -a
  );
}

#[test]
fn inverse_negative() {
  let a = Vector4 {
    x: -1.0,
    y: -1.0,
    z: -1.0,
    w: -1.0,
  };

  assert_eq!(
    Vector4 {
      x: 1.0,
      y: 1.0,
      z: 1.0,
      w: 1.0,
    },
    -a
  );
}

#[test]
fn multiply_by_scalar() {
  let a = Vector4 {
    x: 4.0,
    y: 4.0,
    z: 4.0,
    w: 4.0,
  };

  assert_eq!(
    Vector4 {
      x: 20.0,
      y: 20.0,
      z: 20.0,
      w: 20.0,
    },
    a * 5.0
  );
}

#[test]
fn divide_by_scalar() {
  let a = Vector4 {
    x: 20.0,
    y: 20.0,
    z: 20.0,
    w: 20.0,
  };

  assert_eq!(
    Vector4 {
      x: 4.0,
      y: 4.0,
      z: 4.0,
      w: 4.0,
    },
    a / 5.0
  );
}

#[test]
fn get_magnitude() {
  let a = Vector4 {
    x: 3.0,
    y: 1.0,
    z: -1.0,
    w: -3.0,
  };

  // a.magnitude() should be 4.47214
  // we test the return value of magnitude within 0.00001 of the correct answer
  // because floating point math doesn't need to be any more precise for us
  assert!(a.magnitude() < 4.47215);
  assert!(a.magnitude() > 4.47213);
}

#[test]
fn get_magnitude_zero_vector() {
  let a = Vector4 {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    w: 0.0,
  };

  assert_eq!(a.magnitude(), 0.0);
}

#[test]
fn get_normalize() {
  let a = Vector4::normalize(&mut Vector4 {
    x: 3.0,
    y: 1.0,
    z: -1.0,
    w: -3.0,
  });

  // a.normalize() should be { 0.67082, 0.22360, -0.22360, -0.67082 }
  // we test x and y to be within 0.00001 of the correct answer because
  // floating point math doesn't need to be any more precise for us
  assert!(a.x < 0.67083);
  assert!(a.x > 0.67081);
  assert!(a.y < 0.22361);
  assert!(a.y > 0.22359);
  assert!(a.z < -0.22359);
  assert!(a.z > -0.22361);
  assert!(a.w < -0.67081);
  assert!(a.w > -0.67083);
}
