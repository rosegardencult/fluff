// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Fluff Math Library root entry point
//
//===----------------------------------------------------------------------===//

#![doc(issue_tracker_base_url = "https://github.com/rosegardencult/fluff/issues/")]

#![allow(clippy::needless_return)]

use std::ops::*;

// TODO: Component of ints? and then remove requirement of components in a
//        container all having the same type
// TODO: Restrict T to just numeric types
pub trait Component: Add + Sub + Mul + Copy + Neg + num::Float {}
impl<T> Component for T where T: Add + Sub + Mul + Copy + Neg + num::Float {}

// Vector representations
pub mod vector;
pub use self::vector::vector2::Vector2;
pub use self::vector::vector3::Vector3;
pub use self::vector::vector4::Vector4;

pub mod point;
pub use self::point::point2::Point2;
pub use self::point::point3::Point3;
pub use self::point::point4::Point4;
