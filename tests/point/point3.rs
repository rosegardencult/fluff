// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Point3 test point
//
//===----------------------------------------------------------------------===//

use fluff::Point3;

#[test]
fn to_string() {
  let a = Point3 {
    x: 3.0,
    y: 2.0,
    z: 1.0,
  };
  assert_eq!("Point3 - (x:3, y:2, z:1)", a.to_string());
}

#[test]
fn compare_equal() {
  let a = Point3 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
  };
  let b = Point3 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
  };
  assert!(a == b);
}

#[test]
fn compare_unequal_x() {
  let a = Point3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  let b = Point3 {
    x: 2.0,
    y: 1.0,
    z: 1.0,
  };
  assert!(a != b);
}

#[test]
fn compare_unequal_y() {
  let a = Point3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  let b = Point3 {
    x: 1.0,
    y: 2.0,
    z: 1.0,
  };
  assert!(a != b);
}

#[test]
fn compare_unequal_z() {
  let a = Point3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  let b = Point3 {
    x: 1.0,
    y: 1.0,
    z: 2.0,
  };
  assert!(a != b);
}

#[test]
fn add_point3() {
  let a = Point3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  let b = Point3 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
  };

  assert_eq!(
    Point3 {
      x: 3.0,
      y: 3.0,
      z: 3.0
    },
    a + b
  );
}

#[test]
fn sub_point3() {
  let a = Point3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };
  let b = Point3 {
    x: 2.0,
    y: 2.0,
    z: 2.0,
  };

  assert_eq!(
    Point3 {
      x: -1.0,
      y: -1.0,
      z: -1.0
    },
    a - b
  );
}

#[test]
fn inverse_positive() {
  let a = Point3 {
    x: 1.0,
    y: 1.0,
    z: 1.0,
  };

  assert_eq!(
    Point3 {
      x: -1.0,
      y: -1.0,
      z: -1.0
    },
    -a
  );
}

#[test]
fn inverse_negative() {
  let a = Point3 {
    x: -1.0,
    y: -1.0,
    z: -1.0,
  };

  assert_eq!(
    Point3 {
      x: 1.0,
      y: 1.0,
      z: 1.0
    },
    -a
  );
}

#[test]
fn multiply_by_scalar() {
  let a = Point3 {
    x: 4.0,
    y: 4.0,
    z: 4.0,
  };

  assert_eq!(
    Point3 {
      x: 20.0,
      y: 20.0,
      z: 20.0
    },
    a * 5.0
  );
}

#[test]
fn divide_by_scalar() {
  let a = Point3 {
    x: 20.0,
    y: 20.0,
    z: 20.0,
  };

  assert_eq!(
    Point3 {
      x: 4.0,
      y: 4.0,
      z: 4.0
    },
    a / 5.0
  );
}
